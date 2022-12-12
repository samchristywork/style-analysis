use colored::Colorize;
use std::collections::HashMap;
use tree_sitter::{Language, Node, Parser, TreeCursor};

fn get_node_range(node: Node) -> Vec<(std::ops::Range<usize>, colored::Color)> {
    let mut ret = Vec::new();

    let mut key: HashMap<&str, colored::Color> = HashMap::new();
    key.insert("string_literal", colored::Color::BrightRed);
    key.insert("identifier", colored::Color::BrightMagenta);
    key.insert("fn", colored::Color::White);
    key.insert("!", colored::Color::BrightBlack);

    match key.get(node.kind()) {
        Some(color) => ret.push((node.byte_range(), *color)),
        None => {}
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

pub fn print(src: &str, language: &str) {
    let mut parser = Parser::new();

    extern "C" {
        fn tree_sitter_rust() -> Language;
    }

    extern "C" {
        fn tree_sitter_javascript() -> Language;
    }

    parser
        .set_language(match language {
            "rust" => unsafe { tree_sitter_rust() },
            "javascript" => unsafe { tree_sitter_javascript() },
            _ => panic!("Language ({}) not supported.", language),
        })
        .unwrap();

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
    println!("");
}
