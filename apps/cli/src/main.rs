use code_analyzer_parser_loader::ParserLoader;

fn main() {
    let path_to_plugin = "target/debug/libca_module_example.so";
    let mut parser_loader = ParserLoader::default();
    parser_loader = parser_loader.load_parser(path_to_plugin);
    parser_loader.print_implementations();
}
