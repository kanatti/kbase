# Tree-sitter Integration

## Overview

kbase uses [tree-sitter](https://tree-sitter.github.io/) to parse markdown files. This gives us accurate, structured parsing of:

- **Wikilinks**: `[[note]]`, `[[note|alias]]`, `[[domain/note]]`
- **Tags**: `#rust`, `#deep-dive`, `#nested/tag`
- **Headings**: `# Title`, `## Section`
- **Code blocks**: Properly excluded from tag/link extraction

Tree-sitter is better than regex because it understands markdown's structure. For example, `#tag` inside a code block is not parsed as a tag, and `[[link]]` inside inline code is not parsed as a wikilink.

## The Wrapper Crate

We vendor tree-sitter-markdown with Obsidian extensions in **`crates/tree-sitter-md-obsidian/`**.

The official `tree-sitter-md` crate on crates.io does not have wikilinks or tags enabled. These features are controlled at grammar generation time, so we regenerated the parsers with:
- `EXTENSION_WIKI_LINK=1`
- `EXTENSION_TAGS=1`

This wrapper crate is self-contained (~4.3 MB of generated C code) and requires no external dependencies beyond a C compiler.

## Architecture

Tree-sitter-markdown uses a **dual grammar** approach:

1. **Block grammar** - Parses document structure
   - Headings (`atx_heading`)
   - Code blocks (`fenced_code_block`)
   - Lists, paragraphs, quotes
   - Marks regions as `inline` content

2. **Inline grammar** - Parses inline formatting
   - Bold, italic, links
   - **Wikilinks** (`wiki_link`)
   - **Tags** (`tag`)

You parse in two passes: block structure first, then inline content within marked regions.

## Usage

### Basic Parsing

```rust
use tree_sitter::Parser;
use tree_sitter_md_obsidian::{LANGUAGE, INLINE_LANGUAGE};

// Parse block structure
let mut parser = Parser::new();
parser.set_language(&LANGUAGE.into())?;
let tree = parser.parse(content, None)?;

// Parse inline content
parser.set_language(&INLINE_LANGUAGE.into())?;
let inline_tree = parser.parse(inline_content, None)?;
```

### Querying the Tree

Tree-sitter uses **query files** (`.scm`) to extract specific nodes:

**Query for wikilinks** (`queries/wikilinks.scm`):
```scheme
(wiki_link
  (link_destination) @target
  (link_text)? @alias) @link
```

**Query for tags** (`queries/tags.scm`):
```scheme
(tag) @tag
```

**Query for headings** (`queries/headings.scm`):
```scheme
(atx_heading
  (atx_h1_marker)
  (inline) @text) @heading
```

### Example: Extract Wikilinks

```rust
use tree_sitter::{Parser, Query, QueryCursor};

let query = Query::new(
    &INLINE_LANGUAGE.into(),
    r#"(wiki_link (link_destination) @target)"#
)?;

let mut cursor = QueryCursor::new();
let matches = cursor.matches(&query, tree.root_node(), content.as_bytes());

for m in matches {
    for capture in m.captures {
        let text = &content[capture.node.byte_range()];
        println!("Found wikilink: {}", text);
    }
}
```

## Node Types Reference

### Wikilinks

| Syntax | Node Type | Children |
|--------|-----------|----------|
| `[[note]]` | `wiki_link` | `link_destination` |
| `[[note\|alias]]` | `wiki_link` | `link_destination`, `link_text` |
| `[[domain/note]]` | `wiki_link` | `link_destination` |

### Tags

| Syntax | Node Type | Notes |
|--------|-----------|-------|
| `#rust` | `tag` | Context-aware |
| `#deep-dive` | `tag` | Hyphens allowed |
| `#nested/tag` | `tag` | Slashes allowed |

**Not parsed as tags:**
- `# Heading` (parsed as `atx_heading`)
- `#tag` inside code blocks (excluded by tree structure)
- `#123` (pure numeric)

### Headings

| Syntax | Node Type | Level |
|--------|-----------|-------|
| `# Title` | `atx_heading` + `atx_h1_marker` | 1 |
| `## Section` | `atx_heading` + `atx_h2_marker` | 2 |
| ... | ... | 3-6 |

## Testing

Run the wrapper crate tests:
```bash
cargo test -p tree-sitter-md-obsidian
```

Try the example:
```bash
cargo run -p tree-sitter-md-obsidian --example test_extensions
```

## Updating

To update to a newer version of tree-sitter-markdown:

1. Pull latest upstream:
   ```bash
   cd ~/Code/tree-sitter-markdown
   git pull
   ```

2. Regenerate parsers with extensions:
   ```bash
   cd tree-sitter-markdown
   EXTENSION_WIKI_LINK=1 EXTENSION_TAGS=1 tree-sitter generate
   
   cd ../tree-sitter-markdown-inline
   EXTENSION_WIKI_LINK=1 EXTENSION_TAGS=1 tree-sitter generate
   ```

3. Copy updated files:
   ```bash
   cp tree-sitter-markdown/src/{parser.c,scanner.c} \
      kbase/crates/tree-sitter-md-obsidian/src/block/
   
   cp tree-sitter-markdown-inline/src/{parser.c,scanner.c} \
      kbase/crates/tree-sitter-md-obsidian/src/inline/
   ```

4. Test and update commit hash in wrapper README

## References

- **Wrapper crate**: [`crates/tree-sitter-md-obsidian/README.md`](../crates/tree-sitter-md-obsidian/README.md)
- **Upstream**: [tree-sitter-markdown](https://github.com/tree-sitter-grammars/tree-sitter-markdown)
- **Tree-sitter docs**: [tree-sitter.github.io](https://tree-sitter.github.io/tree-sitter/)
- **Obsidian extensions**: [README Extensions table](https://github.com/tree-sitter-grammars/tree-sitter-markdown#extensions)

