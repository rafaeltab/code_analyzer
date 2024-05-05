use std::env;

use code_analyzer_parser_interface::Parser;

use libloading::{Library, Symbol};

fn main() {
    let cwd = match env::current_dir() {
        Ok(current_dir) => current_dir,
        Err(_) => {
            panic!();
        }
    };
    println!("Starting cli in {}", cwd.to_str().unwrap());

    let path_to_plugin = "target/debug/libca_module_example.so";

    unsafe {

        let lib = Library::new(path_to_plugin).expect("Failed to load plugin");
        let symbol: Symbol<unsafe extern "C" fn () -> *mut dyn Parser> = lib
            .get(b"get_parser_instance")
            .expect("Failed to find symbol");
        let plugin = (symbol)();
        let plugin_ref = plugin.as_ref().expect("Expected parser to exist");
        let language = plugin_ref.get_language();
        let author = plugin_ref.get_author();

        println!("{} by {}", language, author);
    }
}
