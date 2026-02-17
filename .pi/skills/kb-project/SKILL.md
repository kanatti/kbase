---
name: kb-project
description: Reference for working on the kb codebase — a personal knowledge base CLI in Rust. Use when building features, writing tests, reading architecture docs, or understanding the project structure.
---

# kb Project Reference

Personal knowledge base CLI in Rust. Navigates markdown vaults (Obsidian format).

- **Repo:** `~/Code/kb`
- **Vault under development against:** `~/Documents/kanatti-notes` (658 notes, ~52 topics)

## Key Docs

Read these before working on any feature:

| Doc | What it covers |
|-----|---------------|
| `docs/spec.md` | Full command reference — source of truth for UX |
| `docs/implementation.md` | 12-step incremental build plan, current progress |
| `docs/vault.md` | Vault/Index in-memory structures |
| `docs/parsing.md` | Phase 1 (regex) vs Phase 2 (tree-sitter) parsing |
| `docs/search.md` | `--term` search: ripgrep (Phase 1) → Tantivy (Phase 2) |
| `docs/descriptions.md` | How note/topic descriptions are extracted |
| `docs/topics.md` | What a topic is, exclusion rules |
| `docs/config.md` | Config file format and resolution order |

## Source Layout

```
src/
  main.rs       — clap CLI, command dispatch
  config.rs     — Config struct, load/save, KB_CONFIG_DIR
  vault.rs      — Vault, Topic, Note structs + filesystem methods
```

## Current Implementation Status

Done:
- Step 1: `kb config` / `kb config set vault <path>`
- Step 2: `kb topics`, `kb notes`, `kb notes --topic <topic>`

Next:
- Step 3: `kb read <topic>/<note>` — full path only, no ambiguity
- Step 4: `kb outline <note>` — heading tree from a single file
- Step 5: `kb notes --term` — ripgrep-backed content search

See `docs/implementation.md` for full step list.

## Commands Implemented

```bash
kb config                        # show config
kb config set vault <path>       # set vault path
kb topics                        # list topics with note counts
kb topics --sort count           # sort by note count
kb notes                         # list all notes
kb notes --topic <topic>         # list notes in topic
kb notes --files                 # filenames only
kb notes --term <term>           # not yet implemented
```

## Architecture Decisions

- **Error handling:** `anyhow` everywhere, single handler in `main`
- **Vault resolution:** `--vault` flag → `KB_VAULT` env → `~/.kb/config.toml`
- **Config dir override:** `KB_CONFIG_DIR` env (used in tests)
- **Topics:** top-level dirs not starting with `_` or `.`
- **No frontmatter** in most vault notes — metadata is inline bold text
- **Phase 1:** in-memory only, full index built per run
- **Phase 2:** Tantivy index at `~/.kb/index/`, SQLite dropped in favour of Tantivy

## Test Setup

```bash
cargo test                # run all tests
cargo test topics         # run only topics tests
```

Test isolation: each test copies `tests/fixtures/vault/` into a fresh `TempDir`.
Shared helpers in `tests/common/mod.rs` — `setup_vault()`, `kb()`.

Fixture vault layout:
```
tests/fixtures/vault/
  elasticsearch/   (2 notes)
  lucene/          (3 notes)
  rust/            (1 note)
  __templates/     (excluded)
  _logs/           (excluded)
  01-home.md       (root file, not a topic)
```

## Key Cargo Dependencies

```toml
clap        — CLI parsing (derive + env features)
ignore      — vault file walking (gitignore-aware, same author as rg)
walkdir     — directory traversal
regex       — wikilink / tag / task parsing
anyhow      — error handling
toml        — config file parsing
dirs        — home directory resolution
shellexpand — ~ expansion in paths
colored     — terminal output
```

Phase 2 (not yet added):
```toml
tantivy     — full-text search index (see docs/search.md)
```
