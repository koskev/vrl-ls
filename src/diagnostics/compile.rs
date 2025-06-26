use language_server::{cache::Cache, diagnostics::Diagnostics};
use lsp_types::{Diagnostic, DiagnosticSeverity, Position, Range};
use ropey::Rope;
use vrl::diagnostic::Severity;

use crate::ast::VrlAstGenerator;

pub struct CompileDiagnostics<'a> {
    cache: &'a Cache<VrlAstGenerator>,
}

impl<'a> CompileDiagnostics<'a> {
    pub fn new(cache: &'a Cache<VrlAstGenerator>) -> Self {
        Self { cache }
    }
}

pub trait IntoLSPSeverity {
    fn into_severity(&self) -> DiagnosticSeverity;
}

impl IntoLSPSeverity for Severity {
    fn into_severity(&self) -> DiagnosticSeverity {
        match self {
            Severity::Bug => DiagnosticSeverity::ERROR,
            Severity::Error => DiagnosticSeverity::ERROR,
            Severity::Warning => DiagnosticSeverity::WARNING,
            Severity::Note => DiagnosticSeverity::INFORMATION,
        }
    }
}

impl<'a> Diagnostics for CompileDiagnostics<'a> {
    fn diagnostics(&self, filename: &str) -> Vec<lsp_types::Diagnostic> {
        let Some(doc) = self.cache.get_document(filename) else {
            return vec![];
        };
        let res = vrl::compiler::compile(&doc.content, &vrl::stdlib::all());
        let diags = match res {
            Ok(res) => res.warnings,
            Err(e) => e,
        };

        let rope = Rope::from_str(&doc.content);
        diags
            .iter()
            .flat_map(|d| {
                d.labels().iter().map(|l| {
                    let line = rope.char_to_line(l.span.start());
                    let start = Position {
                        line: line as u32,
                        character: (l.span.start() - rope.line_to_char(line)) as u32,
                    };
                    Diagnostic {
                        severity: Some(d.severity.into_severity()),
                        message: l.message.clone(),
                        range: Range { start, end: start },
                        ..Default::default()
                    }
                })
            })
            .collect()
    }
}
