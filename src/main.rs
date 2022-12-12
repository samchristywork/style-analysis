use tree_sitter::{Node, TreeCursor};

pub mod highlight;

fn print_node(node: Node, level: usize) {
    let source_code = "fn test() { println!(\"Hello, World!\"); }";

    print!("{}", " ".repeat(level));
    // println!("{:?}", &source_code[node.start_byte()..node.end_byte()]);
    println!("{:?}", node.kind());
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
    let source_code = "fn test() {\n    print!(\"Hello, \");\n    println!(\"World!\");\n}";
    highlight::print(source_code);
}
