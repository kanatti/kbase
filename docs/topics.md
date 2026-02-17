# Topics

A **topic** is a top-level directory in the vault. Topics are the primary
navigation axis in `kb` — most commands are scoped to one or more topics.

---

## What counts as a topic

Any top-level directory that does NOT start with `_` or `.`:

```
kanatti-notes/
├── elasticsearch/    ← topic
├── lucene/           ← topic
├── datafusion/       ← topic
├── rust/             ← topic
├── arrow/            ← topic
│   ...~50 more...
│
├── __templates/      ← NOT a topic (starts with __)
├── __attachments/    ← NOT a topic (starts with __)
├── __canvas/         ← NOT a topic (starts with __)
├── _logs/            ← NOT a topic (starts with _)
├── _planning/        ← NOT a topic (starts with _)
│
├── 01-home.md        ← NOT a topic (it's a file)
├── CLAUDE.md         ← NOT a topic (it's a file)
```

---

## Topic folder convention

Most topics follow a standard layout:

```
elasticsearch/
├── 01-home.md         # overview, goals, current focus, key links
├── 02-logs.md         # chronological work log
├── 03-task-board.md   # open tasks for this topic
├── 04-archive.md      # completed phases and tasks
└── esql-analysis.md   # content notes (flat, kebab-case names)
```

Not every topic has all four standard files — smaller topics may just have a
few content notes.

---

## Topics in commands

Topics are the primary scoping mechanism:

```bash
kb topics                          # list all topics with note counts
kb ls lucene                       # list notes inside a topic
kb tasks -t elasticsearch          # tasks scoped to a topic
kb search "BKD tree" -t lucene     # search scoped to a topic
kb research lucene datafusion      # multi-topic research session
```

---

## In code

`vault.topics()` returns `Vec<Topic>` by scanning the vault root:

```rust
struct Topic {
    name: String,       // "elasticsearch"
    path: PathBuf,      // /vault/elasticsearch
    note_count: usize,  // number of .md files inside
}
```

Implementation: `read_dir(vault_root)`, keep entries that are directories and
whose name does not start with `_` or `.`, count `.md` files in each.
