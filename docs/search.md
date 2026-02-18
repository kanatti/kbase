# Search & Filtering

How `kb notes` finds notes through different filtering mechanisms.

---

## Interface

All filtering is exposed via `kb notes`, keeping discovery unified:

```bash
kb notes --term "search flow"               # full-text search (future)
kb notes --domain lucene                    # filter by domain
kb notes --tag wip                          # filter by tag ✅ IMPLEMENTED
kb notes --domain lucene --tag deep-dive    # combine filters ✅ IMPLEMENTED
```

This keeps discovery unified — `kb notes` is always how you find notes,
whether you're browsing by domain, filtering by tag, or searching by term.

---

## Tag filtering ✅ IMPLEMENTED

`kb notes --tag <name>` filters notes that contain the specified hashtag:

```bash
kb notes --tag rust          # All notes with #rust tag
kb notes --tag wip           # All notes with #wip tag  
kb notes --domain rust --tag advanced  # Combine with domain filter
```

Uses tag-first filtering for efficiency:
1. Load tag index from `~/.kb/indexes/<vault>/tags.json`
2. Get paths for the specified tag
3. Convert paths to Note structs (only for tagged files)
4. Apply domain filter if specified

Requires `kb index` to build the tag index first.

---

## Full-text search (future)

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
