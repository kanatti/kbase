# Search

How `kb notes --term` finds notes, and the path to Tantivy full-text search.

---

## Interface

Search is exposed via `kb notes --term`, not as a separate command:

```bash
kb notes --term "search flow"               # search across all domains
kb notes --domain lucene --term "search"     # scoped to a domain
```

This keeps discovery unified — `kb notes` is always how you find notes,
whether you're browsing by domain or searching by term.

---

## What `--term` matches

A note matches if the term appears in:
1. The filename (`search-flow.md`)
2. The title (first `# Heading`)
3. The note content (full text)

---

## Phase 1: ripgrep

`--term` uses ripgrep under the hood. No index required — scans files on
every run.

```
kb notes --term "BKD tree"
    → rg -l -i "BKD tree" <vault>
    → collect matching file paths
    → resolve to Note structs
    → display with domain/filename/title
```

Scoped to a domain:
```
kb notes --domain lucene --term "BKD tree"
    → rg -l -i "BKD tree" <vault>/lucene/
```

**Implementation notes:**
- Use `rg --files-with-matches` (`-l`) — returns paths only, no content
- Case-insensitive by default (`-i`)
- Glob `*.md` only (`--glob "*.md"`)
- Invoked via `std::process::Command`
- Falls back gracefully if `rg` is not installed: error with install hint

**Performance:** ~30ms for 658 files. Acceptable for Phase 1.

---

## Phase 2: Tantivy

Tantivy is a full-text search engine in Rust — essentially Lucene for Rust.
BM25 ranking, persistent inverted index, phrase search, fuzzy matching.

Replaces the SQLite FTS5 idea — Tantivy covers full-text search directly,
no need for two separate systems.

### Index location

```
~/.kb/index/        ← Tantivy index directory
```

Built by `kb index`, updated incrementally on each run by comparing file mtimes.

### What gets indexed

| Field | Stored | Indexed |
|-------|--------|---------|
| `path` | yes | no |
| `domain` | yes | yes |
| `title` | yes | yes (boosted) |
| `content` | no | yes |

Title is boosted so notes with the term in the heading rank higher than
notes that only mention it in passing.

### Query flow

```
kb notes --term "BKD tree"
    → open Tantivy index at ~/.kb/index/
    → query: title:"BKD tree"^2 OR content:"BKD tree"
    → collect top-N hits with BM25 scores
    → resolve paths to Note structs
    → display ranked results
```

### Benefits over ripgrep

| | Phase 1 (ripgrep) | Phase 2 (Tantivy) |
|---|---|---|
| Speed | ~30ms (scan) | ~1ms (index lookup) |
| Ranking | none | BM25 |
| Phrase search | yes | yes |
| Fuzzy search | no | yes |
| External dep | `rg` binary | none (pure Rust) |
| Always fresh | yes | requires `kb index` |

### Cargo dependency

```toml
tantivy = "0.22"
```

---

## When to implement Phase 2

Add Tantivy when:
- The full index pipeline is in place (Step 7)
- Ripgrep scan speed becomes noticeable (larger vault, slower disk)
- Ranked results are needed (research sessions with many matches)
