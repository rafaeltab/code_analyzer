use std::str::Utf8Error;

use code_analyzer_parser_interface::{
    language::{Language, LanguageImplementation},
    BuildMetadata, Parser, ParserImplementation, Prerelease, Version,
};

pub struct ExampleParser {
    language: LanguageImplementation,
}

impl Default for ExampleParser {
    fn default() -> Self {
        ExampleParser {
            language: LanguageImplementation {
                language: Language {
                    name: "markdown".to_string(),
                },
                implementation_key: "tree-sitter-markdown".to_string(),
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

impl ParserImplementation<Result<String, Utf8Error>> for ExampleParser {
    fn parse(&self, text: impl AsRef<[u8]>) -> Result<String, Utf8Error> {
        match std::str::from_utf8(text.as_ref()) {
            Ok(val) => Ok(val.to_string()),
            Err(err) => Err(err),
        }
    }
}
