use tree_sitter::{Language, Parser};

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
    println!("{:?}", root_node);
}
