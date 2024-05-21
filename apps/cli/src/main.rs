use code_analyzer_loader::AnalyzerLoader;

fn main() {
    let plugins = [
    ];
    let mut parser_loader = AnalyzerLoader::default();
    for plugin in plugins {
        parser_loader = parser_loader.load_parser(plugin);
    }
    parser_loader.print_implementations();
}
