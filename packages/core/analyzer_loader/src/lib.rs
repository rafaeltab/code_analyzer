use std::mem::ManuallyDrop;

use code_analyzer_parser_interface::Parser;
use libloading::{Library, Symbol};

#[derive(Default)]
pub struct AnalyzerLoader {
    parsers: Vec<Plugin>,
}

struct Plugin {
    #[allow(dead_code)]
    library: Library,
    parser: ManuallyDrop<Box<dyn Parser>>,
}

impl Drop for Plugin {
    fn drop(&mut self) {
        unsafe {
            let raw_ptr = ManuallyDrop::take(&mut self.parser);
            drop(raw_ptr);
        }
    }
}

impl AnalyzerLoader {
    pub fn load_parser(mut self, path: &str) -> Self {
        unsafe {
            self.parsers.push(AnalyzerLoader::load_parser_unsafe(path));
        }
        self
    }

    pub fn print_implementations(&self) {
        for plugin in &self.parsers[..] {
            println!("{}", plugin.parser.get_implementation())
        }
    }

    unsafe fn load_parser_unsafe(path: &str) -> Plugin {
        let lib = Library::new(path).expect("Failed to load plugin");
        let symbol: Symbol<unsafe extern "C" fn() -> *mut dyn Parser> = lib
            .get(b"get_parser_instance")
            .expect("Failed to find symbol get_parser_instance");
        let res = (symbol)();
        Plugin {
            library: lib,
            parser: ManuallyDrop::new(Box::from_raw(res)),
        }
    }
}
