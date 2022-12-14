use std::{env, fs};
use tree_sitter::{Language, Parser};

pub mod case;
pub mod highlight;
pub mod traverse;

fn print_ast(source_code: &str) {
    let mut parser = Parser::new();

    extern "C" {
        fn tree_sitter_rust() -> Language;
    }

    let language = unsafe { tree_sitter_rust() };
    parser.set_language(language).unwrap();

    let tree = parser.parse(source_code.clone(), None).unwrap();
    let root_node = tree.root_node();

    traverse::traverse_tree(root_node.walk(), 0, |n, l| {
        print!("{}", " ".repeat(l));
        println!("{}", n.kind())
    });
}

fn usage(program_name: &str) {
    println!("Usage: {} file language", program_name);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        usage(args[0].as_str());
        return;
    }

    let source_code = fs::read_to_string(args[1].as_str()).unwrap_or_else(|_| {
        usage(args[0].as_str());
        "".to_string()
    });

    print_ast(source_code.as_str());
    highlight::print(source_code.as_str(), args[2].as_str());
}

#[cfg(test)]
mod tests {
    use tree_sitter::{Language, Parser};

    use crate::case;
    use crate::traverse;

    #[test]
    fn test_case() {
        assert_eq!(case::is_camel("someCase"), true);
        assert_eq!(case::is_camel("SomeCase"), false);
        assert_eq!(case::is_pascal("SomeCase"), true);
        assert_eq!(case::is_pascal("someCase"), false);
        assert_eq!(case::is_screaming_snake("SOME_CASE"), true);
        assert_eq!(case::is_screaming_snake("some_case"), false);
        assert_eq!(case::is_uppercase("SOMECASE"), true);
        assert_eq!(case::is_uppercase("SOME_CASE"), false);
        assert_eq!(case::is_kebab_case("some-case"), true);
        assert_eq!(case::is_kebab_case("Some-case"), false);
        assert_eq!(case::is_lowercase("somecase"), true);
        assert_eq!(case::is_lowercase("some case"), false);
        assert_eq!(case::is_snake_case("some_case"), true);
        assert_eq!(case::is_snake_case("some-case"), false);
    }

    #[test]
    fn test_traverse() {
        let source_code = "fn main(){let a=1;}";

        let mut parser = Parser::new();

        extern "C" {
            fn tree_sitter_rust() -> Language;
        }

        let language = unsafe { tree_sitter_rust() };
        parser.set_language(language).unwrap();

        let tree = parser.parse(source_code.clone(), None).unwrap();
        let root_node = tree.root_node();
        let identifiers = traverse::collect_nodes(root_node.walk(), |n| n.kind().eq("identifier"));

        assert_eq!(identifiers.len(), 2);
    }
}
