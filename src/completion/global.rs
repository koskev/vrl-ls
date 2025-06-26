use language_server::{cache::Cache, utils::rope::RopeHelper};
use lsp_types::{CompletionItem, CompletionItemKind, CompletionList};
use ropey::Rope;
use tree_sitter::{Node, Point};

use crate::{ast::VrlAstGenerator, completion::Completion};

pub struct GlobalCompletion<'a> {
    pub cache: &'a Cache<VrlAstGenerator>,
}

impl<'a> GlobalCompletion<'a> {
    pub fn new(cache: &'a Cache<VrlAstGenerator>) -> Self {
        Self { cache }
    }
}

fn get_sibling_or_parent(node: Node) -> Option<Node> {
    match node.prev_sibling() {
        Some(sibling) => Some(sibling),
        None => node.parent(),
    }
}

fn get_node_identifier(node: Node) -> Option<Node> {
    if node.grammar_name() == "assignment" {
        let first_child = node.child(0)?;
        let second_child = first_child.child(0)?;

        if first_child.grammar_name() == "assign_target" && second_child.grammar_name() == "ident" {
            return Some(second_child);
        }
    }
    None
}

impl<'a> Completion for GlobalCompletion<'a> {
    fn complete(&self, location: lsp_types::Position, filename: &str) -> lsp_types::CompletionList {
        let doc = self.cache.get_document(filename).unwrap();
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&tree_sitter_vrl::language()).unwrap();
        let tree = doc.ast_generator.tree.unwrap();
        let rope = Rope::from_str(&doc.content);
        let non_whitespace = rope.get_prev_non_whitespace(rope.get_index(location));
        let location = rope.get_location(non_whitespace).unwrap();

        let sitter_point = Point {
            row: location.line as usize,
            column: location.character as usize,
        };

        // Find the current node and search the tree upwards for any variables
        // Neither a Query nor a cursor would work for going backwards :(
        let Some(found_node) = tree
            .root_node()
            .descendant_for_point_range(sitter_point, sitter_point)
        else {
            return CompletionList::default();
        };

        let mut prev_node = found_node;
        let mut ident_nodes = vec![];
        while let Some(next_parent) = get_sibling_or_parent(prev_node) {
            if let Some(ident_node) = get_node_identifier(next_parent) {
                ident_nodes.push(ident_node);
            }
            prev_node = next_parent;
        }
        let items = ident_nodes
            .iter()
            .map(|node| {
                let start = node.start_position();
                let end = node.end_position();
                let start_char = rope.line_to_char(start.row) + start.column;
                let end_char = rope.line_to_char(end.row) + end.column;
                let name = rope.slice(start_char..end_char).as_str().unwrap();

                CompletionItem {
                    label: name.to_string(),
                    kind: Some(CompletionItemKind::VARIABLE),
                    ..Default::default()
                }
            })
            .collect();

        CompletionList {
            items,
            ..Default::default()
        }
    }
}
