use std::fmt::Display;

use semver::Version;

/// Any kind of language, `english`, `markdown`, `typescript`, `css`, `html`.
#[derive(Debug)]
pub struct Language {
    /// Then full name of the language, such as `markdown` or `typescript`.
    pub name: String,
}

/// An implementation for a language, such as `tree-sitter-markdown`
///
/// Tree sitter markdown example, when authored by rafaeltab:
/// ```rs
/// let impl = LanguageImplementation {
///   
/// }
/// ```
#[derive(Debug)]
pub struct LanguageImplementation {
    /// The language this implementation is for.
    pub language: Language,

    /// The key of the implementation, such as `tree-sitter-markdown`.
    pub implementation_key: String,

    /// The name of the author of this language implementation.
    ///
    /// Prefix with `@` for individual users, companies or groups should not prefix.
    pub author: String,

    /// The version of the underlying language or package.
    pub version: Version,

    /// The version of this implementation
    pub implementation_version: Version,
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Display for LanguageImplementation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let LanguageImplementation {
            language,
            implementation_key,
            author,
            version,
            implementation_version,
        } = self;

        write!(f, "{language} imlementation using {implementation_key}@{version} by {author} with version {implementation_version}")
    }
}
