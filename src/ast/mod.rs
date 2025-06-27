use language_server::cache::{ASTGenerator, ASTNode};

#[derive(Debug, Default, Clone)]
pub struct VrlAstGenerator {}

#[derive(Debug, Default, Clone)]
pub struct VrlAstNode {
    pub tree: Option<tree_sitter::Tree>,
}

impl ASTNode for VrlAstNode {}

impl ASTGenerator for VrlAstGenerator {
    type Node = VrlAstNode;
    fn update_ast(&self, new_content: &str) -> anyhow::Result<Self::Node> {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&tree_sitter_vrl::language()).unwrap();
        let tree = parser.parse(new_content, None).unwrap();

        Ok(VrlAstNode { tree: Some(tree) })
    }
}
