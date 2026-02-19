# Domain Descriptions

Each domain can have an optional short description shown alongside its name.

## Source

A `_description.md` file in the domain folder. The `_` prefix marks it as
metadata — it won't appear in `kb notes` output.

Description is extracted from `_description.md` using the same rules as notes:
1. YAML frontmatter `description` field, if present
2. Otherwise, first paragraph after the `# Heading`

If `_description.md` doesn't exist, description is `None` and the column is omitted.

## Domain struct

```rust
pub struct Domain {
    pub name: String,
    pub note_count: usize,
    pub description: Option<String>,
}
```

## Display

```
kb domains
  lucene          27 notes   Core full-text search library internals
  elasticsearch   18 notes   Distributed search built on Lucene
  rust             3 notes
```

## Status

Not yet implemented.
