use tree_sitter::Node;

pub fn get_sibling_or_parent(node: Node) -> Option<Node> {
    match node.prev_sibling() {
        Some(sibling) => Some(sibling),
        None => node.parent(),
    }
}

pub fn get_node_identifier(node: Node) -> Option<Node> {
    if node.grammar_name() == "assignment" {
        let first_child = node.child(0)?;
        let second_child = first_child.child(0)?;

        if first_child.grammar_name() == "assign_target" && second_child.grammar_name() == "ident" {
            return Some(second_child);
        }
    }
    None
}
