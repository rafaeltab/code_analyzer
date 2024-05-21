use tree_sitter::Parser;

fn parse(text: impl AsRef<[u8]>) {
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_md::language())
        .expect("Failed loading markdown grammar");
    let tree = parser.parse(text, None).unwrap();
}
