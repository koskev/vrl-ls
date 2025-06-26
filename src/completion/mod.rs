use lsp_types::Position;

pub mod global;
pub mod std;

pub trait Completion {
    fn complete(&self, location: Position, filename: &str) -> lsp_types::CompletionList;
}
