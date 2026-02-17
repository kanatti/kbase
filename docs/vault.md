# Vault

`Vault` is the core abstraction in `kb`. It holds the root path and owns the
in-memory index — a parsed, queryable view of every note in the vault. All
commands go through `Vault`.

---

## In-Memory Structure

Built once on first use via `OnceCell`, cached for the lifetime of the process.

```
Vault
├── root: PathBuf
└── index: OnceCell<Index>
    ├── topics: Vec<Topic>
    ├── notes: Vec<Note>
    ├── link_graph: LinkGraph
    ├── tag_map: TagMap
    └── tasks: Vec<Task>
```

### Topic

Represents a top-level folder in the vault.

```rust
struct Topic {
    name: String,       // "elasticsearch"
    path: PathBuf,      // /vault/elasticsearch
    note_count: usize,  // number of .md files inside
}
```

### Note

Represents a single parsed markdown file.

```rust
struct Note {
    path: PathBuf,           // relative to vault root: "elasticsearch/esql-analysis.md"
    title: String,           // first # heading, or filename if none
    headings: Vec<Heading>,  // all headings in order
    links: Vec<String>,      // raw wikilink targets: ["esql-parser", "01-home"]
    tags: Vec<String>,       // inline tags: ["deep-dive", "wip"]
    tasks: Vec<Task>,        // tasks found in this note
    modified: SystemTime,    // file mtime, used for incremental rebuild
}

struct Heading {
    level: u8,    // 1-6
    text: String, // heading text without #
    line: usize,  // line number in file
}
```

### LinkGraph

Built from all notes' `links` after full vault parse. Resolves raw wikilink
names to actual note paths.

```rust
struct LinkGraph {
    // "elasticsearch/esql-analysis.md" → ["elasticsearch/esql-parser.md", ...]
    forward: HashMap<PathBuf, Vec<PathBuf>>,

    // "elasticsearch/esql-parser.md" → ["elasticsearch/esql-analysis.md", ...]
    reverse: HashMap<PathBuf, Vec<PathBuf>>,

    // notes with broken links: "elasticsearch/03-task-board.md" → ["esql-optimizer"]
    unresolved: HashMap<PathBuf, Vec<String>>,
}
```

Link resolution: given raw link `[[esql-parser]]`:
1. Try `{current_topic}/esql-parser.md`
2. Try `**/esql-parser.md` anywhere in vault (first match)
3. If neither found → unresolved

### TagMap

```rust
struct TagMap {
    // "deep-dive" → ["lucene/search-flow.md", "elasticsearch/esql-analysis.md"]
    tags: HashMap<String, Vec<PathBuf>>,
}
```

### Task

```rust
struct Task {
    path: PathBuf,       // which note this task lives in
    line: usize,         // line number, used for `kb task done`
    status: TaskStatus,  // Todo, Done, Cancelled, Other(char)
    content: String,     // "Complete esql-analysis documentation"
}

enum TaskStatus {
    Todo,        // [ ]
    Done,        // [x]
    Cancelled,   // [-]
    Other(char), // [?] etc.
}
```

---

## How the Index Is Built

On first access to `vault.index()`:

```
1. Walk vault root (using `ignore` crate — respects excludes)
2. For each .md file:
   a. Read file
   b. Parse headings (regex: ^#{1,6} text)
   c. Parse links (regex: \[\[([^\]]+)\]\])
   d. Parse tags (regex: #[a-zA-Z][a-zA-Z0-9_-]*)
   e. Parse tasks (regex: - \[(.)\] (.+))
   f. Build Note struct
3. Collect all Topics from top-level dirs
4. Build LinkGraph by resolving all raw links across notes
5. Build TagMap from all notes' tags
6. Collect all Tasks from all notes
```

Full build on 658 files: ~100-150ms. Cached after that.

---

## On-Disk Persistence (Phase 2)

Phase 1 builds the index in memory on every run. Phase 2 persists it to
`~/.kb/index.db` (SQLite FTS5) so subsequent runs skip the build entirely.

```
~/.kb/
├── config.toml    # vault path and settings
└── index.db       # persisted index (Phase 2)
```

The SQLite schema mirrors the in-memory structures — see `docs/spec.md` for
the full schema.

### Incremental Rebuild

On startup (Phase 2):
1. Load index from `~/.kb/index.db`
2. For each `.md` file in vault, compare `mtime` against stored `modified`
3. Re-parse only changed files, update their rows in the DB
4. Rebuild `LinkGraph` and `TagMap` from updated data

### `kb index` Command

Forces a full rebuild regardless of mtimes:

```bash
kb index           # incremental — only re-parses changed files
kb index --rebuild # force full rebuild from scratch
kb index --status  # show: last built, note count, index size
```

---

## Excludes

The `ignore` crate handles traversal. These paths are always excluded:

| Path | Excluded from |
|---|---|
| `__templates/` | everything |
| `__attachments/` | everything |
| `__canvas/` | everything |
| `_logs/` | search (included in navigation) |
| `_planning/` | search (included in navigation) |

`--logs` and `--planning` flags override the search exclusions.
