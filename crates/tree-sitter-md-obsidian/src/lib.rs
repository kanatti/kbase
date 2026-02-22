//! tree-sitter-markdown with Obsidian extensions enabled
//!
//! This crate provides Markdown language support for the [tree-sitter][] parsing library
//! with the following Obsidian extensions enabled:
//! - `EXTENSION_WIKI_LINK` - Wikilinks: `[[note]]`, `[[note|alias]]`
//! - `EXTENSION_TAGS` - Tags: `#tag` (context-aware, excludes code blocks)
//!
//! It contains two grammars:
//! - [`LANGUAGE`] - Block structure (headings, lists, code blocks, paragraphs)
//! - [`INLINE_LANGUAGE`] - Inline content (bold, links, wikilinks, tags)
//!
//! ## Usage
//!
//! ```rust
//! use tree_sitter::Parser;
//! use tree_sitter_md_obsidian::{LANGUAGE, INLINE_LANGUAGE};
//!
//! let mut parser = Parser::new();
//! parser.set_language(&LANGUAGE.into()).unwrap();
//!
//! let tree = parser.parse("# Hello\n\nSee [[other-note]] #tag", None).unwrap();
//! ```
//!
//! [tree-sitter]: https://tree-sitter.github.io/

use tree_sitter_language::LanguageFn;

unsafe extern "C" {
    fn tree_sitter_markdown() -> *const ();
    fn tree_sitter_markdown_inline() -> *const ();
}

/// The tree-sitter [`LanguageFn`] for the block grammar.
///
/// Parses block-level structure: headings, lists, code blocks, paragraphs.
pub const LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_markdown) };

/// The tree-sitter [`LanguageFn`] for the inline grammar.
///
/// Parses inline content: bold, italic, links, **wikilinks**, **tags**.
///
/// ## Obsidian Extensions
///
/// This grammar includes:
/// - **Wikilinks**: `[[note]]`, `[[note|alias]]`, `[[domain/note]]`
/// - **Tags**: `#tag`, `#nested/tag` (context-aware, excludes code blocks)
pub const INLINE_LANGUAGE: LanguageFn =
    unsafe { LanguageFn::from_raw(tree_sitter_markdown_inline) };

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_load_block_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&LANGUAGE.into())
            .expect("Error loading Markdown block grammar");
    }

    #[test]
    fn can_load_inline_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&INLINE_LANGUAGE.into())
            .expect("Error loading Markdown inline grammar");
    }

    #[test]
    fn test_wikilink_parsing() {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&INLINE_LANGUAGE.into()).unwrap();

        let tree = parser.parse("[[test-note]]", None).unwrap();
        let root = tree.root_node();
        let sexp = root.to_sexp();

        assert!(sexp.contains("wiki_link"), "Should parse wikilink node");
    }

    #[test]
    fn test_tag_parsing() {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&INLINE_LANGUAGE.into()).unwrap();

        let tree = parser.parse("#rust", None).unwrap();
        let root = tree.root_node();
        let sexp = root.to_sexp();

        assert!(sexp.contains("tag"), "Should parse tag node");
    }
}
