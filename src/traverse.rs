use tree_sitter::{Node, TreeCursor};

fn traverse_helper<F: Fn(Node, usize)>(cursor: &mut TreeCursor, depth: usize, f: &F) {
    f(cursor.node(), depth);

    if cursor.goto_first_child() {
        loop {
            traverse_helper(cursor, depth + 1, f);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        cursor.goto_parent();
    }
}

pub fn traverse_tree<F: Fn(Node, usize)>(mut cursor: TreeCursor, depth: usize, f: F) {
    traverse_helper(&mut cursor, depth, &f);
}

pub fn collect_nodes<F: Fn(Node) -> bool>(
    mut cursor: TreeCursor,
    collection_criterion: F,
) -> Vec<Node> {
    let mut ret = Vec::new();

    loop {
        if collection_criterion(cursor.node()) {
            ret.push(cursor.node());
        }

        if cursor.goto_first_child() {
            continue;
        }

        if cursor.goto_next_sibling() {
            continue;
        }

        loop {
            if !cursor.goto_parent() {
                return ret;
            }

            if cursor.goto_next_sibling() {
                break;
            }
        }
    }
}
