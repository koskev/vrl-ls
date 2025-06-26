use language_server::cache::Cache;
use lsp_types::{CompletionItem, CompletionItemKind, CompletionList};
use ropey::Rope;

use crate::{ast::VrlAstGenerator, completion::Completion};

pub struct GlobalCompletion<'a> {
    pub cache: &'a Cache<VrlAstGenerator>,
}

impl<'a> GlobalCompletion<'a> {
    pub fn new(cache: &'a Cache<VrlAstGenerator>) -> Self {
        Self { cache }
    }
}

impl<'a> Completion for GlobalCompletion<'a> {
    fn complete(&self, location: lsp_types::Position, filename: &str) -> lsp_types::CompletionList {
        let doc = self.cache.get_document(filename).unwrap();
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&tree_sitter_vrl::language()).unwrap();
        let query = tree_sitter::Query::new(
            &parser.language().unwrap(),
            "(assignment (assign_target (ident) @variable))",
        )
        .unwrap();
        let mut cursor = tree_sitter::QueryCursor::new();
        let tree = doc.ast_generator.tree.unwrap();
        let captures = cursor.captures(&query, tree.root_node(), doc.content.as_bytes());
        let rope = Rope::from_str(&doc.content);
        let items = captures
            .flat_map(|(q_match, _)| {
                q_match
                    .captures
                    .iter()
                    // TODO: poor version of in_range :)
                    .filter(|capture| capture.node.start_position().row < location.line as usize)
                    .map(|capture| {
                        let start = capture.node.start_position();
                        let end = capture.node.end_position();
                        let start_char = rope.line_to_char(start.row) + start.column;
                        let end_char = rope.line_to_char(end.row) + end.column;
                        let name = rope.slice(start_char..end_char).as_str().unwrap();

                        CompletionItem {
                            label: name.to_string(),
                            kind: Some(CompletionItemKind::VARIABLE),
                            ..Default::default()
                        }
                    })
            })
            .collect();
        CompletionList {
            items,
            ..Default::default()
        }
    }
}
