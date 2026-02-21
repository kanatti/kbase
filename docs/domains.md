# Domains

A **domain** is a top-level directory in the vault. Domains organize notes by topic or project area.

## List domains

```bash
kb domains                  # list all domains sorted by name
kb domains --sort count     # sort by note count (descending)
kb domains --sort name      # sort by name (explicit)
```

Shows a table with domain names and note counts:

```
Domain           Notes
elasticsearch    2
lucene           3
rust             1
```

If any domain has a description, the output includes a third column:

```
Domain           Notes  Description
elasticsearch    2      Search and analytics engine
lucene           3      Full-text search library
rust             1      
```

If the vault has no domains, shows "No domains found in vault."

## Domain descriptions

Add a description to any domain by creating `_description.md` or `description.md` in the domain directory. Descriptions can be multi-line markdown.

Priority order:
1. `_description.md` (checked first)
2. `description.md` (fallback)

## What counts as a domain

Any top-level directory that does NOT start with `_` or `.`:

```
kanatti-notes/
├── elasticsearch/    ← domain
├── lucene/           ← domain
├── rust/             ← domain
│
├── __templates/      ← NOT a domain (starts with __)
├── _logs/            ← NOT a domain (starts with _)
│
├── 01-home.md        ← NOT a domain (it's a file)
```

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

## Using domains in commands

Many commands accept a `--domain` filter:

```bash
kb notes --domain lucene           # list notes in a domain
kb notes --tag deep-dive --domain lucene  # combine with tags
```
