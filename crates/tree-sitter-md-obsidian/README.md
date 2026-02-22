# tree-sitter-md-obsidian

A vendored build of [tree-sitter-markdown](https://github.com/tree-sitter-grammars/tree-sitter-markdown) with **Obsidian extensions enabled**.

## What is this?

This is a minimal Rust wrapper around tree-sitter-markdown's C parsers, compiled with:
- `EXTENSION_WIKI_LINK=1` - Enables Obsidian wikilinks `[[note]]`, `[[note|alias]]`
- `EXTENSION_TAGS=1` - Enables Obsidian tags `#tag` (context-aware)

## Why vendor this?

The official `tree-sitter-md` crate on crates.io does **not** have these extensions enabled. 

The extensions are controlled at **grammar generation time** (when `parser.c` is created from `grammar.js`), not at compile time. This means:
- We can't enable them via Cargo features
- We can't enable them via environment variables during `cargo build`
- The only way is to regenerate the parser with extensions enabled

## How was this generated?

```bash
# 1. Clone tree-sitter-markdown
git clone https://github.com/tree-sitter-grammars/tree-sitter-markdown.git
cd tree-sitter-markdown

# 2. Regenerate block grammar with extensions
cd tree-sitter-markdown
EXTENSION_WIKI_LINK=1 EXTENSION_TAGS=1 tree-sitter generate

# 3. Regenerate inline grammar with extensions
cd ../tree-sitter-markdown-inline
EXTENSION_WIKI_LINK=1 EXTENSION_TAGS=1 tree-sitter generate

# 4. Copy generated C files to kbase
cd ../../kbase
cp ~/tree-sitter-markdown/tree-sitter-markdown/src/parser.c crates/tree-sitter-md-obsidian/src/block/
cp ~/tree-sitter-markdown/tree-sitter-markdown/src/scanner.c crates/tree-sitter-md-obsidian/src/block/
cp ~/tree-sitter-markdown/tree-sitter-markdown-inline/src/parser.c crates/tree-sitter-md-obsidian/src/inline/
cp ~/tree-sitter-markdown/tree-sitter-markdown-inline/src/scanner.c crates/tree-sitter-md-obsidian/src/inline/
```

## File sizes

```
src/block/parser.c    ~2.0 MB
src/block/scanner.c   ~58 KB
src/inline/parser.c   ~2.2 MB
src/inline/scanner.c  ~16 KB
─────────────────────────────
Total:                ~4.3 MB
```

## Structure

```
tree-sitter-md-obsidian/
├── Cargo.toml          # Minimal dependencies
├── build.rs            # Compiles C files with cc crate
├── README.md           # This file
└── src/
    ├── lib.rs          # FFI bindings to expose LANGUAGE and INLINE_LANGUAGE
    ├── block/          # Block-level parsing (headings, lists, code blocks)
    │   ├── parser.c
    │   └── scanner.c
    └── inline/         # Inline parsing (bold, links, wikilinks, tags)
        ├── parser.c
        └── scanner.c
```

## Upstream source

- **Original repository**: https://github.com/tree-sitter-grammars/tree-sitter-markdown
- **Commit used**: `cee71b8` (2024-02-21)
- **Extensions reference**: See [tree-sitter-markdown README](https://github.com/tree-sitter-grammars/tree-sitter-markdown#extensions)

## License

MIT (same as tree-sitter-markdown)

## Updating

To regenerate with a newer version of tree-sitter-markdown:

1. Pull latest tree-sitter-markdown
2. Run `tree-sitter generate` with extension flags in both subdirectories
3. Copy the updated `parser.c` and `scanner.c` files
4. Test with `cargo test`
5. Update this README with the new commit hash
