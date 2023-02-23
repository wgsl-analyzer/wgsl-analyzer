use tree_sitter::Parser;

pub fn parse_compare(code: &str) {
    let mut parser = Parser::new();
    let language = tree_sitter_wgsl::language();
    parser.set_language(language).unwrap();
    let tree = parser.parse(code, None).unwrap();

    let should_error = tree.root_node().has_error();
    if !should_error {
        let parse = wgsl_parser::parse_file(code);
        if !parse.errors().is_empty() {
            panic!(
                "Shouldn't error for code {code}, errors {:?}",
                parse.errors()
            );
        }
    }
}
