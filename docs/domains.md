# Domains

A **domain** is a top-level directory in the vault. Domains are the primary
navigation axis in `kb` — most commands are scoped to one or more domains.

---

## What counts as a domain

Any top-level directory that does NOT start with `_` or `.`:

```
kanatti-notes/
├── elasticsearch/    ← domain
├── lucene/           ← domain
├── datafusion/       ← domain
├── rust/             ← domain
├── arrow/            ← domain
│   ...~50 more...
│
├── __templates/      ← NOT a domain (starts with __)
├── __attachments/    ← NOT a domain (starts with __)
├── __canvas/         ← NOT a domain (starts with __)
├── _logs/            ← NOT a domain (starts with _)
├── _planning/        ← NOT a domain (starts with _)
│
├── 01-home.md        ← NOT a domain (it's a file)
├── CLAUDE.md         ← NOT a domain (it's a file)
```

---

## Domain folder convention

Most domains follow a standard layout:

```
elasticsearch/
├── 01-home.md         # overview, goals, current focus, key links
├── 02-logs.md         # chronological work log
├── 03-task-board.md   # open tasks for this domain
├── 04-archive.md      # completed phases and tasks
└── esql-analysis.md   # content notes (flat, kebab-case names)
```

Not every domain has all four standard files — smaller domains may just have a
few content notes.

---

## Domains in commands

Domains are the primary scoping mechanism:

```bash
kb domains                         # list all domains with note counts
kb ls lucene                       # list notes inside a domain
kb tasks -d elasticsearch          # tasks scoped to a domain
kb search "BKD tree" -d lucene     # search scoped to a domain
kb research lucene datafusion      # multi-domain research session
```

---

## In code

`vault.domains()` returns `Vec<Domain>` by scanning the vault root:

```rust
struct Domain {
    name: String,       // "elasticsearch"
    path: PathBuf,      // /vault/elasticsearch
    note_count: usize,  // number of .md files inside
}
```

Implementation: `read_dir(vault_root)`, keep entries that are directories and
whose name does not start with `_` or `.`, count `.md` files in each.
