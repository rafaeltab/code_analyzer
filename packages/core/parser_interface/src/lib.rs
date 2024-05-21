use code_analyzer_common::language::{Language, LanguageImplementation};
pub use code_analyzer_common::*;
pub use semver::{BuildMetadata, Prerelease, Version};

pub trait Parser {
    fn get_language(&self) -> &Language;
    fn get_author(&self) -> &str;
    fn get_implementation(&self) -> &LanguageImplementation;
}

pub trait ParserImplementation<TResult> {
    fn parse(&self, text: impl AsRef<[u8]>) -> TResult;
}
