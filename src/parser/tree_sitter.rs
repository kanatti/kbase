//! Tree-sitter based Markdown parser for Obsidian-flavored notes.
//!
//! ## How Query-Based Extraction Works
//!
//! Tree-sitter queries pattern-match against the CST (Concrete Syntax Tree) to extract semantic elements.
//! The key concept: **One Match = One Element** with multiple captures.
//!
//! ### Example: Heading Extraction
//!
//! For content `# Hello World`:
//!
//! ```text
//! Match {
//!     captures: [
//!         Capture { name: "@h1", node: <entire atx_heading> },
//!         Capture { name: "@text", node: <inline "Hello World"> }
//!     ]
//! }
//! ```
//!
//! Both captures are in the same Match object. The inner loop processes both,
//! setting `level = 1` and `text = "Hello World"`, then creates one Heading.
//!
//! ### Visual Flow for Multiple Headings
//!
//! ```text
//! Content: "# Hello\n## World"
//!
//! ┌─────────────────────┐
//! │ Match #1            │  ← First heading
//! │  captures:          │
//! │    @h1 → level=1    │  ← Both from same match
//! │    @text → "Hello"  │
//! └─────────────────────┘
//!         ↓
//!     Heading { level: 1, text: "Hello", line: 1 }
//!
//! ┌─────────────────────┐
//! │ Match #2            │  ← Second heading
//! │  captures:          │
//! │    @h2 → level=2    │  ← Both from same match
//! │    @text → "World"  │
//! └─────────────────────┘
//!         ↓
//!     Heading { level: 2, text: "World", line: 2 }
//! ```
//!
//! ### Why This Works
//!
//! The tree-sitter query structurally binds the captures:
//!
//! ```scm
//! (atx_heading
//!   (atx_h1_marker)
//!   (inline) @text) @h1
//! ```
//!
//! This says: "Capture the whole `atx_heading` as `@h1` AND its `inline` child as `@text`."
//! Since both captures are in the same pattern, they're guaranteed to correspond.

use super::queries::Queries;
use super::{Heading, MarkdownParser, ParsedMarkdown, Wikilink};
use anyhow::{Context, Result};
use std::collections::BTreeSet;
use tree_sitter::{Language, Node, Parser, QueryCursor, StreamingIterator, Tree};
use tree_sitter_md_obsidian::{INLINE_LANGUAGE, LANGUAGE};

/// Helper to create a parser with a language set
fn new_parser(language: &Language) -> Result<Parser> {
    let mut parser = Parser::new();
    parser
        .set_language(language)
        .context("Failed to set language for parser")?;
    Ok(parser)
}

/// Helper to extract text from a node's byte range
fn extract_text<'a>(content: &'a str, node: &Node) -> &'a str {
    &content[node.start_byte()..node.end_byte()]
}

/// Context holding parsed trees and content for extraction.
/// This allows us to pass all necessary data together.
struct ExtractionContext<'a> {
    block_tree: &'a Tree,
    content: &'a str,
}

impl<'a> ExtractionContext<'a> {
    fn new(block_tree: &'a Tree, content: &'a str) -> Self {
        Self {
            block_tree,
            content,
        }
    }
}

pub struct TreeSitterParser {
    block_parser: Parser,
    inline_parser: Parser,
    queries: Queries,
}

impl TreeSitterParser {
    pub fn new() -> Result<Self> {
        let block_lang = &LANGUAGE.into();
        let inline_lang = &INLINE_LANGUAGE.into();

        Ok(Self {
            block_parser: new_parser(block_lang)?,
            inline_parser: new_parser(inline_lang)?,
            queries: Queries::compile(block_lang, inline_lang)?,
        })
    }

    fn extract_headings(&mut self, ctx: &ExtractionContext) -> Result<Vec<Heading>> {
        let mut headings = Vec::new();
        let mut cursor = QueryCursor::new();
        let root_node = ctx.block_tree.root_node();

        // Each match represents ONE heading with MULTIPLE captures (@h1 + @text).
        // We accumulate data from all captures in a match, then build one Heading.
        // See module docs for detailed explanation with diagrams.
        let mut matches = cursor.matches(&self.queries.headings, root_node, ctx.content.as_bytes());
        while let Some(match_) = matches.next() {
            let mut level = 0u8;
            let mut text = String::new();
            let mut line = 0usize;

            for capture in match_.captures {
                let node = capture.node;
                let capture_name = &self.queries.headings.capture_names()[capture.index as usize];

                match capture_name.as_ref() {
                    "h1" => level = 1,
                    "h2" => level = 2,
                    "h3" => level = 3,
                    "h4" => level = 4,
                    "h5" => level = 5,
                    "h6" => level = 6,
                    "text" => {
                        text = extract_text(ctx.content, &node).trim().to_string();
                        line = node.start_position().row + 1; // 1-indexed
                    }
                    _ => {}
                }
            }

            if level > 0 && !text.is_empty() {
                headings.push(Heading { level, text, line });
            }
        }

        Ok(headings)
    }

    /// Extract both tags and wikilinks in one pass.
    /// Parses inline content once and queries for both element types.
    fn extract_inline_elements(
        &mut self,
        ctx: &ExtractionContext,
    ) -> Result<(Vec<String>, Vec<Wikilink>)> {
        // Parse the entire content as inline
        // The inline grammar is context-aware and won't match tags/links in code blocks
        let inline_tree = self
            .inline_parser
            .parse(ctx.content, None)
            .context("Failed to parse inline content")?;

        // Extract both element types from the same tree
        let tags = self.extract_tags_from_tree(&inline_tree, ctx.content)?;
        let wikilinks = self.extract_wikilinks_from_tree(&inline_tree, ctx.content)?;

        Ok((tags, wikilinks))
    }

    /// Extract tags from an inline tree.
    fn extract_tags_from_tree(&self, inline_tree: &Tree, content: &str) -> Result<Vec<String>> {
        let mut tags = BTreeSet::new();
        let mut cursor = QueryCursor::new();
        let root = inline_tree.root_node();

        let mut matches = cursor.matches(&self.queries.tags, root, content.as_bytes());
        while let Some(match_) = matches.next() {
            for capture in match_.captures {
                let node = capture.node;
                let tag_text = extract_text(content, &node);

                // Remove the # prefix
                if let Some(tag_name) = tag_text.strip_prefix('#') {
                    tags.insert(tag_name.to_string());
                }
            }
        }

        Ok(tags.into_iter().collect())
    }

    /// Extract wikilinks from an inline tree.
    fn extract_wikilinks_from_tree(
        &self,
        inline_tree: &Tree,
        content: &str,
    ) -> Result<Vec<Wikilink>> {
        let mut wikilinks = Vec::new();
        let mut cursor = QueryCursor::new();
        let root = inline_tree.root_node();

        let mut matches = cursor.matches(&self.queries.wikilinks, root, content.as_bytes());
        while let Some(match_) = matches.next() {
            let mut target = String::new();
            let mut alias: Option<String> = None;
            let mut line = 0usize;
            let mut column = 0usize;

            for capture in match_.captures {
                let node = capture.node;
                let capture_name = &self.queries.wikilinks.capture_names()[capture.index as usize];

                match capture_name.as_ref() {
                    "target" => {
                        target = extract_text(content, &node).to_string();
                    }
                    "alias" => {
                        alias = Some(extract_text(content, &node).to_string());
                    }
                    "link" => {
                        let pos = node.start_position();
                        line = pos.row + 1; // 1-indexed
                        column = pos.column;
                    }
                    _ => {}
                }
            }

            if !target.is_empty() {
                // Handle [[note#section]] syntax - split on #
                let (target_path, section) = if let Some(hash_pos) = target.find('#') {
                    let (path, sect) = target.split_at(hash_pos);
                    (path.to_string(), Some(sect[1..].to_string())) // Skip the #
                } else {
                    (target, None)
                };

                wikilinks.push(Wikilink {
                    target: target_path,
                    alias,
                    section,
                    line,
                    column,
                });
            }
        }

        Ok(wikilinks)
    }
}

impl MarkdownParser for TreeSitterParser {
    fn parse(&mut self, content: &str) -> Result<ParsedMarkdown> {
        // Parse block structure
        let block_tree = self
            .block_parser
            .parse(content, None)
            .context("Failed to parse block structure")?;

        // Create extraction context with parsed tree and content
        let ctx = ExtractionContext::new(&block_tree, content);

        // Extract headings from block tree
        let headings = self.extract_headings(&ctx)?;

        // Extract tags and wikilinks from inline content in one pass
        let (tags, wikilinks) = self.extract_inline_elements(&ctx)?;

        // Determine title: first H1 heading, or empty string
        let title = headings.first().map(|h| h.text.clone()).unwrap_or_default();

        Ok(ParsedMarkdown {
            title,
            headings,
            wikilinks,
            tags,
            body: content.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_creation() {
        let parser = TreeSitterParser::new();
        assert!(parser.is_ok());
    }

    #[test]
    fn test_parse_simple_note() {
        let content = "# Main Title\n\nThis is a test note.\n";

        let mut parser = TreeSitterParser::new().unwrap();
        let parsed = parser.parse(content).unwrap();

        assert_eq!(parsed.title, "Main Title");
        assert_eq!(parsed.headings.len(), 1);
        assert_eq!(parsed.headings[0].level, 1);
        assert_eq!(parsed.headings[0].text, "Main Title");
    }

    #[test]
    fn test_parse_with_tags() {
        let content = "# Test\n\nThis has #rust and #deep-dive tags.\n";

        let mut parser = TreeSitterParser::new().unwrap();
        let parsed = parser.parse(content).unwrap();

        assert_eq!(parsed.tags, vec!["deep-dive", "rust"]);
    }

    #[test]
    fn test_parse_with_wikilinks() {
        let content = "See [[other-note]] for details.\nAlso [[domain/note|Display Name]].\n";

        let mut parser = TreeSitterParser::new().unwrap();
        let parsed = parser.parse(content).unwrap();

        assert_eq!(parsed.wikilinks.len(), 2);
        assert_eq!(parsed.wikilinks[0].target, "other-note");
        assert_eq!(parsed.wikilinks[0].alias, None);
        assert_eq!(parsed.wikilinks[1].target, "domain/note");
        assert_eq!(parsed.wikilinks[1].alias, Some("Display Name".to_string()));
    }

    #[test]
    fn test_wikilink_with_section() {
        let content = "See [[note#Introduction]] for details.\n";

        let mut parser = TreeSitterParser::new().unwrap();
        let parsed = parser.parse(content).unwrap();

        assert_eq!(parsed.wikilinks.len(), 1);
        assert_eq!(parsed.wikilinks[0].target, "note");
        assert_eq!(
            parsed.wikilinks[0].section,
            Some("Introduction".to_string())
        );
    }

    #[test]
    fn test_tags_not_in_code_blocks() {
        let content = r#"Normal text with #real-tag.

```rust
// This #fake-tag should be ignored
```

After code: #another-real.
"#;

        let mut parser = TreeSitterParser::new().unwrap();
        let parsed = parser.parse(content).unwrap();

        // Should NOT include "fake-tag"
        assert!(!parsed.tags.contains(&"fake-tag".to_string()));
        assert!(parsed.tags.contains(&"real-tag".to_string()));
        assert!(parsed.tags.contains(&"another-real".to_string()));
    }

    #[test]
    fn test_multiple_headings() {
        let content = r#"# Main Title

## Section 1

### Subsection

## Section 2
"#;

        let mut parser = TreeSitterParser::new().unwrap();
        let parsed = parser.parse(content).unwrap();

        assert_eq!(parsed.title, "Main Title");
        assert_eq!(parsed.headings.len(), 4);
        assert_eq!(parsed.headings[1].level, 2);
        assert_eq!(parsed.headings[1].text, "Section 1");
        assert_eq!(parsed.headings[2].level, 3);
        assert_eq!(parsed.headings[2].text, "Subsection");
    }

    #[test]
    fn test_no_title() {
        let content = "Some content without a heading.\n";

        let mut parser = TreeSitterParser::new().unwrap();
        let parsed = parser.parse(content).unwrap();

        assert_eq!(parsed.title, "");
        assert_eq!(parsed.headings.len(), 0);
    }
}
