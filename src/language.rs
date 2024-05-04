use semver::Version;

/// Any kind of language, `english`, `markdown`, `typescript`, `css`, `html`.
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
    pub implementation_version: Version
}
