use anyhow::anyhow;
use language_server::{
    cache::Cache,
    completion::Completion,
    diagnostics::Diagnostics,
    server::{LSPConnection, LSPError, LSPServer},
};
use lsp_types::{
    CompletionList, CompletionResponse, Diagnostic, InlayHint, InlayHintLabel, OneOf, Position,
    ServerCapabilities, TextDocumentSyncKind, TextDocumentSyncOptions,
};
use ropey::Rope;
use tree_sitter::Node;

use crate::{
    ast::VrlAstGenerator,
    completion::{
        global::GlobalCompletion,
        std::{StdCompletion, StdFunction, StdFunctions},
    },
    diagnostics::compile::CompileDiagnostics,
};

#[derive(Default)]
pub struct VRLServer {
    pub connection: LSPConnection,
    pub cache: Cache<VrlAstGenerator>,

    pub std_completion: StdCompletion,
}

impl LSPServer for VRLServer {
    type AstGenerator = VrlAstGenerator;
    fn connection(&self) -> &LSPConnection {
        &self.connection
    }

    fn cache(&self) -> &Cache<Self::AstGenerator> {
        &self.cache
    }
    fn get_diagnostics(&self, filename: &str) -> Vec<Diagnostic> {
        CompileDiagnostics::new(&self.cache).diagnostics(filename)
    }

    fn get_capabilities(&self) -> lsp_types::ServerCapabilities {
        ServerCapabilities {
            text_document_sync: Some(lsp_types::TextDocumentSyncCapability::Options(
                TextDocumentSyncOptions {
                    open_close: Some(true),
                    change: Some(TextDocumentSyncKind::INCREMENTAL),
                    ..Default::default()
                },
            )),
            completion_provider: Some(lsp_types::CompletionOptions {
                ..Default::default()
            }),
            inlay_hint_provider: Some(OneOf::Left(true)),
            ..Default::default()
        }
    }

    fn completion(
        &self,
        params: lsp_types::CompletionParams,
    ) -> anyhow::Result<language_server::server::LSPResponse, LSPError> {
        let mut lists = vec![];
        lists.push(GlobalCompletion::new(&self.cache).complete(
            params.text_document_position.position,
            params.text_document_position.text_document.uri.as_str(),
        ));
        lists.push(self.std_completion.complete(
            params.text_document_position.position,
            params.text_document_position.text_document.uri.as_str(),
        ));
        let failed: Vec<_> = lists.iter().filter_map(|res| res.as_ref().err()).collect();
        let succeeded: Vec<&CompletionList> =
            lists.iter().filter_map(|res| res.as_ref().ok()).collect();

        if succeeded.len() == 0 && failed.len() > 0 {
            let first_err = *failed.first().unwrap();
            return Err(first_err.into());
        }

        for err in failed {
            log::error!("Failed to complete: {}", err)
        }

        let is_incomplete = succeeded.iter().any(|list| list.is_incomplete);
        let completion_list = CompletionList {
            items: succeeded
                .into_iter()
                .flat_map(|list| list.items.clone())
                .collect(),
            is_incomplete,
        };

        Ok(CompletionResponse::List(completion_list).into())
    }

    fn inlay_hint(
        &self,
        params: <lsp_types::request::InlayHintRequest as lsp_types::request::Request>::Params,
    ) -> anyhow::Result<language_server::server::LSPResponse, LSPError> {
        let doc = self.cache.get_document(params.text_document.uri.as_str())?;

        let tree = doc
            .get_ast()?
            .tree
            .clone()
            .ok_or(anyhow!("AST was never parsed"))?;
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&tree.language())
            .or_else(|e| Err(anyhow!("Setting language of parser: {e}")))?;
        let query = tree_sitter::Query::new(&tree.language(), "(function_call (arguments) @call)")
            .expect("BUG: Invalid query");
        let mut cursor = tree_sitter::QueryCursor::new();
        let captures = cursor.captures(&query, tree.root_node(), doc.content.as_bytes());
        let rope = Rope::from_str(&doc.content);
        let std_functions = StdFunctions::generate();
        let items: Vec<InlayHint> = captures
            .flat_map(|(q_match, _)| {
                q_match.captures.iter().flat_map(|capture| {
                    let Some(std_func) = get_std_function(capture.node, &rope, &std_functions)
                    else {
                        return vec![];
                    };
                    let Some(arg_nodes) = get_argument_nodes(capture.node) else {
                        return vec![];
                    };

                    arg_nodes
                        .iter()
                        .enumerate()
                        .filter_map(|(i, node)| {
                            Some(InlayHint {
                                label: InlayHintLabel::String(format!(
                                    "{}:",
                                    std_func.arguments.get(i)?.name
                                )),
                                position: Position {
                                    line: node.start_position().row as u32,
                                    character: node.start_position().column as u32,
                                },
                                kind: None,
                                text_edits: None,
                                tooltip: None,
                                padding_right: Some(true),
                                padding_left: None,
                                data: None,
                            })
                        })
                        .collect::<Vec<InlayHint>>()
                })
            })
            .collect();

        Ok(items.into())
    }
}

fn get_std_function<'a>(
    call_node: Node,
    rope: &'a Rope,
    std_functions: &'a StdFunctions,
) -> Option<&'a StdFunction> {
    let mut prev_sibling = call_node.prev_sibling()?;
    if prev_sibling.grammar_name() == "!" {
        prev_sibling = prev_sibling.prev_sibling()?;
    }
    let function_name = get_node_name(&prev_sibling, &rope);
    Some(std_functions.functions.get(&function_name.to_string())?)
}

fn get_argument_nodes(call_node: Node) -> Option<Vec<Node>> {
    let mut arg_nodes = vec![];
    let mut arg_cursor = call_node.walk();
    // This is (
    arg_cursor.goto_first_child();
    while arg_cursor.goto_next_sibling() && arg_cursor.node().grammar_name() != ")" {
        if arg_cursor.node().grammar_name() != "," {
            arg_nodes.push(arg_cursor.node());
        }
    }

    Some(arg_nodes)
}

fn get_node_name<'a>(node: &'a Node, rope: &'a Rope) -> &'a str {
    let start = node.start_position();
    let end = node.end_position();
    let start_char = rope.line_to_char(start.row) + start.column;
    let end_char = rope.line_to_char(end.row) + end.column;
    rope.slice(start_char..end_char)
        .as_str()
        .unwrap_or_default()
}
