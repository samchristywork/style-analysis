use tree_sitter::{Node, TreeCursor};

fn helper<F: Fn(Node, usize)>(cursor: &mut TreeCursor, depth: usize, f: &F) {
    f(cursor.node(), depth);

    if cursor.goto_first_child() {
        loop {
            helper(cursor, depth + 1, f);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }
}

pub fn traverse_tree<F: Fn(Node, usize)>(mut cursor: TreeCursor, depth: usize, f: F) {
    helper(&mut cursor, depth, &f);
}
