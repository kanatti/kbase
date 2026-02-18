# Tags

How kb discovers, indexes, and queries tags across the knowledge base.

---

## Overview

Tags in kb work as metadata labels that help categorize and discover notes. They integrate with `kb notes` command as a filter.

```bash
kb notes --tag deep-dive                    # notes with this tag
kb notes --tag wip --domain lucene          # tag + domain filter  
kb tags                                     # list all available tags
kb index                                    # build tag index
```

Tags are extracted from inline content: `#tag` anywhere in note body (excluding code blocks).

---

## Tag Sources

### Inline Tags

Hash-prefixed tags anywhere in note content:

```markdown
# BKD Trees

This is a #deep-dive into Lucene's #indexing architecture.

The #performance characteristics are...
```

**Rules:**
- Tags inside code blocks (``` fenced) are ignored
- Only alphanumeric + underscore/hyphen: `#valid-tag`, `#tag_name`
- Must start with letter: `#tag123` ✅, `#123tag` ❌

---

## Tag Index

### Storage Location

```
~/.kb/indexes/<vault-name>/tags.json
```

### Format

Simple JSON mapping from tag names to note paths:

```json
{
  "deep-dive": [
    "lucene/search-flow.md",
    "elasticsearch/esql-analysis.md"
  ],
  "wip": [
    "lucene/codec-study.md"
  ]
}
```

---

## Commands

### `kb tags`

List all tags with usage counts.

```bash
kb tags                          # all tags, sorted by name
kb tags --sort count             # sort by usage frequency
```

**Output:**
```
deep-dive (2 notes)
performance (1 note)  
wip (1 note)
```

### `kb notes --tag`

Filter notes by tag.

```bash
kb notes --tag deep-dive                      # all notes with tag
kb notes --tag deep-dive --domain lucene      # tag + domain filter
kb notes --tag wip --files                    # tag filter + filename-only output  
```

**Output** follows `kb notes` format:
```
lucene/search-flow.md      Search Flow Deep Dive
elasticsearch/esql-analysis.md    ESQL Analysis
```

### `kb index`

Build the tag index by scanning the vault.

```bash
kb index                         # build tag index (and others)
```

**Output:**
```
Building indexes for vault 'personal'...
Tag index built: 47 tags across 658 notes
```

---

## Error Handling

### Missing Index

```bash
kb notes --tag deep-dive
# No tag index found. Run `kb index` to build it first.
```

### Tag Not Found

```bash
kb notes --tag nonexistent
# No notes with tag 'nonexistent'.
```

---

## Implementation

Tag-first filtering for efficiency:
1. Load tag index from `tags.json`
2. Get paths for the specified tag
3. Convert paths to Note structs (only for tagged files)
4. Apply domain filter if specified

This avoids scanning all files when filtering by tag.