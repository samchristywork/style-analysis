use colored::Colorize;
use tree_sitter::{Language, Node, Parser, TreeCursor};

fn get_node_range(node: Node) -> Vec<(std::ops::Range<usize>, colored::Color)> {
    let mut ret = Vec::new();

    if node.kind().eq("string_literal") {
        ret.push((node.byte_range(), colored::Color::BrightMagenta));
    }
    if node.kind().eq("identifier") {
        ret.push((node.byte_range(), colored::Color::BrightGreen));
    }
    if node.kind().eq("fn") {
        ret.push((node.byte_range(), colored::Color::Yellow));
    }
    if node.kind().eq("!") {
        ret.push((node.byte_range(), colored::Color::Blue));
    }

    ret
}

fn traverse_tree(mut cursor: TreeCursor) -> Vec<(std::ops::Range<usize>, colored::Color)> {
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
        let mut color = colored::Color::White;

        for j in ranges.clone() {
            if j.0.contains(&k) {
                found = true;
                color = j.1;
                break;
            }
        }
        if found {
            print!("{}", format!("{}", i).color(color));
        } else {
            print!("{}", i);
        }

        k += 1;
    }
}
