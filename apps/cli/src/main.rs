use code_analyzer_module::language::{Language, LanguageImplementation};
use semver::{BuildMetadata, Prerelease, Version};

fn main() {
    let implementation = LanguageImplementation {
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
    };

    println!("{}", implementation);
}
