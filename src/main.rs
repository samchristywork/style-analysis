use tree_sitter::{Language, Parser};

pub mod highlight;
pub mod traverse;

fn main() {
    let source_code = "fn test() {\n    print!(\"Hello, \");\n    println!(\"World!\");\n}";
    highlight::print(source_code);

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
}
