use code_analyzer_parser_interface::{language::{Language, LanguageImplementation}, BuildMetadata, Parser, Prerelease, Version};

#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub extern "C" fn get_parser_instance() -> *mut dyn Parser {
    let parser: Box<dyn Parser> = Box::new(ExampleParser::new());
    Box::into_raw(parser)
}

struct ExampleParser {
    language: LanguageImplementation,
}

impl ExampleParser {
    fn new() -> Self {
        ExampleParser {
            language: LanguageImplementation {
                language: Language {
                    name: "example".to_string(),
                },
                implementation_key: "ca_example".to_string(),
                author: "@rafaeltab".to_string(),
                version: Version {
                    major: 0,
                    minor: 2,
                    patch: 3,
                    pre: Prerelease::EMPTY,
                    build: BuildMetadata::EMPTY,
                },
                implementation_version: Version {
                    major: 0,
                    minor: 0,
                    patch: 1,
                    pre: Prerelease::EMPTY,
                    build: BuildMetadata::EMPTY,
                },
            },
        }
    }
}

impl Parser for ExampleParser {
    fn get_language(&self) -> &Language {
        &self.language.language
    }

    fn get_author(&self) -> &str {
        &self.language.author
    }

    fn get_implementation(&self) -> &LanguageImplementation {
        &self.language
    }
}
