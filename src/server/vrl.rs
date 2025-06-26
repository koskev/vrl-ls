use language_server::{
    cache::Cache,
    diagnostics::Diagnostics,
    server::{LSPConnection, LSPServer},
};
use lsp_types::{
    CompletionResponse, Diagnostic, ServerCapabilities, TextDocumentSyncKind,
    TextDocumentSyncOptions,
};

use crate::{
    ast::VrlAstGenerator,
    completion::{Completion, std::StdCompletion},
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

            ..Default::default()
        }
    }

    fn completion(
        &self,
        params: lsp_types::CompletionParams,
    ) -> anyhow::Result<language_server::server::LSPResponse, lsp_server::ResponseError> {
        let completion_list = StdCompletion::new().complete(
            params.text_document_position.position,
            params.text_document_position.text_document.uri.as_str(),
        );
        Ok(CompletionResponse::List(completion_list).into())
    }
}
