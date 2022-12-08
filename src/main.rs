use tree_sitter::{Language, Node, Parser, TreeCursor};

fn print_node(node: Node, level: usize) {
    let source_code = "fn test() { println!(\"Hello, World!\"); }";

    print!("{}", " ".repeat(level));
    println!("{:?}", &source_code[node.start_byte()..node.end_byte()]);
}

fn traverse_tree(mut cursor: TreeCursor, level: usize) {
    if !cursor.goto_first_child() {
        return;
    }
    print_node(cursor.node(), level);

    loop {
        print_node(cursor.node(), level);

        traverse_tree(cursor.clone(), level + 1);

        if !cursor.goto_next_sibling() {
            break;
        }
    }
}

fn main() {
    let mut parser = Parser::new();

    extern "C" {
        fn tree_sitter_rust() -> Language;
    }

    let language = unsafe { tree_sitter_rust() };
    parser.set_language(language).unwrap();

    let source_code = "fn test() { println!(\"Hello, World!\"); }";
    let tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();

    traverse_tree(root_node.walk(), 0);
}
