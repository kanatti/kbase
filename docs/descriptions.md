# Descriptions

How `kb` extracts a short description for topics and notes, used in `kb ls`
and anywhere a summary is needed.

---

## What a description is

A one-line (or short) summary of what a note or topic is about. Not the full
content — just enough to distinguish it at a glance.

```
lucene/
  01-home.md              Lucene                        Overview of Lucene internals and current focus areas.
  search-flow.md          Search Flow Deep Dive         How TermQuery flows through IndexSearcher into TopDocs.
  codecs.md               Codecs Deep Dive              PackedInts, ForUtil, and the codec abstraction layer.
```

---

## Source of truth

Two sources, tried in order:

### 1. YAML frontmatter `description` field

```yaml
---
description: "Deep dive into ES|QL parsing and analysis pipeline"
---
# ES|QL Analysis
```

Wins if present. Use this when you want an explicit, curated description that
is independent of the note's opening prose.

### 2. First paragraph after the `# Heading`

```markdown
# Search Flow Deep Dive

How TermQuery flows through IndexSearcher, Weight, Scorer and into TopDocs.

## Phase 1
...
```

The paragraph immediately after the top-level heading, if it exists and is
plain prose (not a heading, not a task list, not a code block). Trimmed to
a single line for display.

If neither source exists, description is `None` — commands that show
descriptions just omit the column.

---

## For topics

Topic description comes from `01-home.md` in the topic folder, using the same
resolution order above. If `01-home.md` does not exist, description is `None`.

---

## In code

`Note` and `Topic` both get an `Option<String>` description field:

```rust
struct Note {
    path: PathBuf,
    filename: String,
    title: String,
    description: Option<String>,  // ← added
}

struct Topic {
    name: String,
    path: PathBuf,
    note_count: usize,
    description: Option<String>,  // ← added, sourced from 01-home.md
}
```

Extraction function (used for both):

```rust
fn extract_description(content: &str) -> Option<String> {
    // 1. Try YAML frontmatter
    if let Some(desc) = parse_frontmatter_description(content) {
        return Some(desc);
    }
    // 2. First paragraph after # heading
    first_paragraph_after_heading(content)
}
```

---

## When to implement

This is not part of the initial `kb ls` (Step 2). It will be added when:
- The basic parser is in place (Step 7)
- Or as a lightweight pre-Step-7 addition if descriptions are needed earlier

The display in `kb ls` should degrade gracefully — if description is `None`,
just show title and filename as today, no empty column.

---

## Display

Descriptions are shown in `kb ls` by default, truncated to fit the terminal
width. A `--no-desc` flag hides them if the output is too wide.

```
kb ls lucene              # shows filename + title + description (truncated)
kb ls lucene --no-desc    # shows filename + title only (current behaviour)
kb ls lucene --files      # shows filenames only (no title, no description)
```
