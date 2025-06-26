use language_server::cache::ASTGenerator;

#[derive(Debug, Default, Clone)]
pub struct VrlAstGenerator {}

impl ASTGenerator for VrlAstGenerator {
    fn update_ast(&mut self, new_content: &str) -> anyhow::Result<()> {
        Ok(())
    }
}
