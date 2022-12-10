use tree_sitter::{Node, TreeCursor};

fn print_node(node: Node, src: &str) {
    // let source_code = "fn test() { println!(\"Hello, World!\"); }";

    // print!("{}", " ".repeat(level));
    if node.child_count() == 0 {
        println!("{}", &src[node.start_byte()..node.end_byte()]);
    }
    // println!("{:?}", node.kind());
}

pub fn traverse_tree(mut cursor: TreeCursor, src: &str) {
    if !cursor.goto_first_child() {
        return;
    }
    print_node(cursor.node(), src);

    loop {
        print_node(cursor.node(), src);

        traverse_tree(cursor.clone(), src);

        if !cursor.goto_next_sibling() {
            break;
        }
    }
}
