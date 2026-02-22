pub mod queries;
pub mod tree_sitter;
pub mod types;

pub use tree_sitter::TreeSitterParser;
pub use types::{Heading, ParsedMarkdown, Wikilink};

use anyhow::Result;

/// Trait for markdown parsers. Allows swapping implementations.
pub trait MarkdownParser {
    fn parse(&mut self, content: &str) -> Result<ParsedMarkdown>;
}
