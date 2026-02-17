# kb index

How `kb index` builds and stores the knowledge base index.

---

## Overview

`kb index` does a single full-rebuild pass over every note in the vault.
No incremental updates — run it whenever the vault changes.

```bash
kb index
# Indexed 658 notes (lucene: 27, elasticsearch: 18, ...)
```

Three structures are produced and saved under `~/.kb/vaults/<name>/`:

```
~/.kb/
  config.toml
  vaults/
    personal/
      index/        ← Tantivy full-text search index
      tags.json     ← tag → [note paths]
      links.json    ← note path → [resolved note paths]
    work/
      index/
      tags.json
      links.json
```

Each vault has its own isolated index directory. The vault name comes from
the `[vaults]` table in `~/.kb/config.toml`.

Each structure is independent — search, tags, and links are separate use cases
with different query patterns and different representations.

---

## Tantivy Full-Text Index

### Purpose

Powers `kb notes --term <query>` with BM25-ranked full-text search.

### Location

```
~/.kb/index/      ← Tantivy index directory (multiple files managed by Tantivy)
```

### Schema

| Field     | Type    | Stored | Indexed | Boost |
|-----------|---------|--------|---------|-------|
| `path`    | TEXT    | yes    | no      | —     |
| `topic`   | TEXT    | yes    | yes     | 1×    |
| `title`   | TEXT    | yes    | yes     | 2×    |
| `content` | TEXT    | no     | yes     | 1×    |

- `path` — vault-relative path (`lucene/search-flow.md`). Stored for retrieval, not searchable.
- `topic` — folder name (`lucene`). Stored + indexed for topic-scoped queries.
- `title` — first `# Heading` or filename stem. Boosted 2× so title matches rank higher than body matches.
- `content` — full raw markdown text. Indexed only (not stored); not needed after search.

### Query flow

```
kb notes --term "BKD tree"
  → open index at ~/.kb/index/
  → query: title:"BKD tree"^2 OR content:"BKD tree"
  → collect top-N scored hits
  → return stored path + title fields
  → display ranked results (path column, title column)

kb notes --term "BKD tree" --topic lucene
  → same query + topic:"lucene" filter
```

### In memory

`tantivy::Index` opened read-only during search, write-only during `kb index`.
No persistent in-memory state — index is opened per command invocation.

---

## Tags Map

### Purpose

Powers `kb tags` (list all tags with counts) and `kb tag <name>` (notes with a given tag).

### Location

```
~/.kb/tags.json
```

### On-disk format

```json
{
  "codec":      ["lucene/codecs-deep-dive.md", "lucene/postings-format.md"],
  "wip":        ["lucene/study-plan.md"],
  "deep-dive":  ["lucene/search-flow-deep-dive.md", "elasticsearch/esql-analysis.md"]
}
```

A plain JSON object: `{ tag: [vault-relative paths] }`.
Paths are sorted alphabetically within each tag list.
Tags are sorted alphabetically at the top level.

### In memory

```rust
HashMap<String, Vec<String>>  // tag → sorted list of vault-relative paths
```

Loaded fully into memory on demand (tags.json is small — a few KB even for large vaults).

### Tag sources

Two sources, merged and deduplicated per note:

1. **YAML frontmatter** — `tags:` field (list or space-separated string):
   ```yaml
   ---
   tags: [codec, deep-dive]
   ---
   ```
   ```yaml
   ---
   tags: codec deep-dive
   ---
   ```

2. **Inline body tags** — `#tag` anywhere in the note body (not in code blocks):
   ```markdown
   This covers the #codec architecture and #deep-dive internals.
   ```

Tags are normalized to lowercase. The `#` prefix is stripped from inline tags.
Tags from frontmatter and body are merged per-note; duplicates are dropped.

---

## Link Graph

### Purpose

Powers `kb links <note>` (outgoing links), `kb backlinks <note>` (incoming links),
`kb orphans` (no links in or out), `kb deadends` (links to non-existent notes).

### Location

```
~/.kb/links.json
```

### On-disk format

```json
{
  "lucene/search-flow-deep-dive.md": [
    "lucene/codecs-deep-dive.md",
    "lucene/postings-format.md"
  ],
  "lucene/study-plan.md": [
    "lucene/search-flow-deep-dive.md"
  ]
}
```

A plain JSON object: `{ source-path: [resolved-target-paths] }`.
Only notes with at least one outgoing link appear as keys.
Paths are vault-relative. Unresolved links are omitted.

### In memory

```rust
HashMap<String, Vec<String>>  // source path → sorted list of resolved target paths
```

Backlinks are derived by inverting this map at query time — no separate backlinks file.

### Wikilink resolution

Obsidian wikilinks take the form `[[target]]` or `[[target|alias]]`.
Resolution turns a raw target string into a vault-relative path.

**Resolution algorithm:**

Given a wikilink target `t` found in note `source`:

1. **Path-style target** — `t` contains `/` (e.g. `lucene/codecs`):
   - Try `vault/{t}.md` directly.
   - If exists → resolved. Done.
   - If not → unresolved (do not fall back).

2. **Bare name target** — `t` has no `/` (e.g. `codecs`):
   - Try `{source_topic}/{t}.md` (same-topic relative lookup).
   - If exists → resolved. Done.
   - Try all topics: find any note whose stem matches `t`.
   - If exactly one match → resolved. Done.
   - If multiple matches → ambiguous → unresolved (log a warning during `kb index`).
   - If no match → unresolved.

**Unresolved links** are silently dropped from `links.json` but counted and
reported at the end of `kb index`:

```
Indexed 658 notes
  27 unresolved wikilinks (run `kb deadends` for details)
```

**Section links** (`[[note#section]]`) — the `#section` part is stripped before
resolution. Only the note target is recorded in the graph.

---

## Parsed Note (intermediate, not persisted)

The parser produces this for each note during `kb index`. It is consumed
immediately to feed the three structures above and is not saved to disk.

```rust
pub struct ParsedNote {
    pub path: PathBuf,            // vault-relative
    pub topic: Option<String>,    // folder name, or None for root notes
    pub title: String,            // first # heading, or filename stem
    pub tags: Vec<String>,        // merged frontmatter + inline, normalized
    pub headings: Vec<Heading>,   // all headings with level + text
    pub wikilinks: Vec<RawLink>,  // unresolved raw targets from [[...]]
    pub body: String,             // full raw content for Tantivy
}

pub struct Heading {
    pub level: u8,
    pub text: String,
}

pub struct RawLink {
    pub target: String,           // raw target string before resolution
    pub alias: Option<String>,    // [[target|alias]] — alias part
}
```

---

## Disk Layout Summary

```
~/.kb/
  config.toml
  vaults/
    personal/
      index/              ← Tantivy manages this directory
        meta.json
        .managed.json
        <segment files>
      tags.json           ← written atomically on each kb index run
      links.json          ← written atomically on each kb index run
    work/
      index/
      tags.json
      links.json
```

Both JSON files are written atomically (write to `.tmp`, then rename)
to avoid leaving partial files if `kb index` is interrupted.

The index directory for the active vault is: `~/.kb/vaults/<vault-name>/`

When `KB_VAULT` env var is set (path override, used in tests), the index
directory is derived by hashing the path:
`~/.kb/vaults/_override_<hash>/` — isolated per path, never collides with named vaults.

---

## `kb index` Behavior

```bash
kb index              # index the default vault
kb --vault work index # index a named vault
```

1. Resolve vault (KB_VAULT → --vault name → config default).
2. Create `~/.kb/vaults/<name>/` if it does not exist.
3. Walk all notes (same rules as `kb notes` — all topics, all `.md` files).
4. For each note: parse → feed parser output to Tantivy writer, tags accumulator, links accumulator.
5. Commit Tantivy index.
6. Write `tags.json` and `links.json` atomically.
7. Print summary.

**If no vault is configured:**
```
Error: no vaults configured. Run `kb vault add <name> <path>` to get started.
```

**If index directory cannot be created:**
```
Error: could not create index directory: <path>: <os error>
```

**If `--term` used without an index:**
```
Error: no index found for this vault. Run `kb index` first.
```
