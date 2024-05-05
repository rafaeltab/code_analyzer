use code_analyzer_common::language::Language;
pub use code_analyzer_common::*;
pub use semver::{Version,Prerelease,BuildMetadata};

pub trait Parser {
    fn get_language(&self) -> &Language;
    fn get_author(&self) -> &str;
}
