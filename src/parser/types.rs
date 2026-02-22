use std::path::PathBuf;

/// Parsed markdown content with extracted structured data.
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedMarkdown {
    /// First H1 heading, or empty if none
    pub title: String,

    /// All headings in document order
    pub headings: Vec<Heading>,

    /// All wikilinks in document order
    pub wikilinks: Vec<Wikilink>,

    /// Unique tags, sorted alphabetically
    pub tags: Vec<String>,

    /// Full markdown content
    pub body: String,
}

/// A note with both parsed markdown content and file metadata.
/// Reserved for future use when we integrate parsing more deeply into indexing.
#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedNote {
    /// Vault-relative path
    pub path: PathBuf,

    /// Parsed markdown content
    pub markdown: ParsedMarkdown,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Heading {
    pub level: u8, // 1-6
    pub text: String,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Wikilink {
    pub target: String,          // "domain/note" or "note"
    pub alias: Option<String>,   // Display text if [[target|alias]]
    pub section: Option<String>, // Heading if [[target#section]]
    pub line: usize,
    pub column: usize,
}
