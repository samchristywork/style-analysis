use colored::Colorize;
use tree_sitter::{Language, Node, Parser, TreeCursor};

fn get_node_range(node: Node) -> Vec<std::ops::Range<usize>> {
    let mut ret = Vec::new();

    if node.kind().eq("string_literal") {
        ret.push(node.byte_range());
    }

    ret
}

fn traverse_tree(mut cursor: TreeCursor) -> Vec<std::ops::Range<usize>> {
    let mut ret = Vec::new();
    if !cursor.goto_first_child() {
        return ret;
    }
    ret.append(&mut get_node_range(cursor.node()));

    loop {
        ret.append(&mut get_node_range(cursor.node()));

        ret.append(&mut traverse_tree(cursor.clone()));

        if !cursor.goto_next_sibling() {
            break;
        }
    }

    ret
}

pub fn print(src: &str) {
    let mut parser = Parser::new();

    extern "C" {
        fn tree_sitter_rust() -> Language;
    }

    let language = unsafe { tree_sitter_rust() };
    parser.set_language(language).unwrap();

    let tree = parser.parse(src, None).unwrap();
    let root_node = tree.root_node();

    let ranges = traverse_tree(root_node.walk());

    let mut k = 0;
    for i in src.chars() {
        let mut found = false;

        for j in ranges.clone() {
            if j.contains(&k) {
                found = true;
                break;
            }
        }
        if found {
            print!("{}", format!("{}", i).magenta());
        } else {
            print!("{}", i);
        }

        k += 1;
    }
}
