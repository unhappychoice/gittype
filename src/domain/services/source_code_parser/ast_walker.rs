use tree_sitter::{Node, Tree};

pub trait ASTVisitor {
    fn visit_node(&mut self, node: Node, source_code: &str);
    fn enter_node(&mut self, _node: Node, _source_code: &str) {}
    fn exit_node(&mut self, _node: Node, _source_code: &str) {}
}

pub struct ASTWalker;

impl ASTWalker {
    pub fn walk<V: ASTVisitor>(visitor: &mut V, tree: &Tree, source_code: &str) {
        Self::walk_node(visitor, tree.root_node(), source_code);
    }

    fn walk_node<V: ASTVisitor>(visitor: &mut V, node: Node, source_code: &str) {
        visitor.enter_node(node, source_code);
        visitor.visit_node(node, source_code);

        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                Self::walk_node(visitor, cursor.node(), source_code);
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }

        visitor.exit_node(node, source_code);
    }
}
