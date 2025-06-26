use language_server::cache::ASTGenerator;
use tree_sitter::ffi::ts_query_cursor_exec;

#[derive(Debug, Default, Clone)]
pub struct VrlAstGenerator {
    pub tree: Option<tree_sitter::Tree>,
}

impl ASTGenerator for VrlAstGenerator {
    fn update_ast(&mut self, new_content: &str) -> anyhow::Result<()> {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&tree_sitter_vrl::language()).unwrap();
        let tree = parser.parse(new_content, None).unwrap();
        self.tree = Some(tree);

        Ok(())
    }
}
