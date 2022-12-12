use std::fs;
use tree_sitter::{Language, Parser};

pub mod highlight;
pub mod traverse;

fn main() {
    let source_code = fs::read_to_string("input.rs").unwrap();
    highlight::print(source_code.as_str());

    let mut parser = Parser::new();

    extern "C" {
        fn tree_sitter_rust() -> Language;
    }

    let language = unsafe { tree_sitter_rust() };
    parser.set_language(language).unwrap();

    let tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();

    traverse::traverse_tree(root_node.walk(), 0, |n, l| {
        print!("{}", " ".repeat(l));
        println!("{}", n.kind())
    });

    println!(
        "{:?}",
        traverse::collect_nodes(root_node.walk(), |n| n.kind().eq("string_literal"))
    );
}
