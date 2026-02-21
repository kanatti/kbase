---
name: kbase-development
description: Reference for working on the kbase codebase — a personal knowledge base CLI in Rust. Use when building features, writing tests, reading architecture docs, or understanding the project structure.
---

# kbase Development Reference

Personal knowledge base CLI in Rust. Navigates markdown vaults (Obsidian format).

- **Repo:** `~/Code/kbase`
- **Binary:** `kbase`
- **Vault under development against:** `~/Documents/kanatti-notes` (658 notes, ~52 domains)

## Key Docs

Read these before working on any feature:

| Doc | What it covers |
|-----|---------------|
| `docs/spec.md` | Full command reference — source of truth for UX |
| `docs/implementation.md` | 12-step incremental build plan, current progress |
| `docs/vault.md` | Vault/Index in-memory structures |
| `docs/parsing.md` | Phase 1 (regex) vs Phase 2 (tree-sitter) parsing |
| `docs/search.md` | `--term` search: ripgrep (Phase 1) → Tantivy (Phase 2) |
| `docs/descriptions.md` | How note/domain descriptions are extracted |
| `docs/domains.md` | What a domain is, exclusion rules |
| `docs/config.md` | Config file format and resolution order |

## Source Layout

```
src/
  main.rs       — clap CLI, command dispatch
  config.rs     — Config struct, load/save, KB_HOME
  vault.rs      — Vault, Domain, Note structs + filesystem methods
```

## Current Implementation Status

Done:
- Step 1: `kbase config` / `kbase config set vault <path>`
- Step 2: `kbase domains`, `kbase notes`, `kbase notes --domain <domain>`

Next:
- Step 3: `kbase read <domain>/<note>` — full path only, no ambiguity
- Step 4: `kbase outline <note>` — heading tree from a single file
- Step 5: `kbase notes --term` — ripgrep-backed content search

See `docs/implementation.md` for full step list.

## Commands Implemented

```bash
kbase config                        # show config
kbase config set vault <path>       # set vault path
kbase domains                        # list domains with note counts (recursive)
kbase domains --sort count           # sort by note count
kbase notes                         # list all notes (recursive)
kbase notes --domain <domain>         # list notes in domain (includes subdirectories)
kbase notes --files                 # filenames only
kbase notes --term <term>           # not yet implemented
```

**Note:** Domains are still top-level directories only, but note counts and listings include all nested subdirectories recursively.

## Architecture Decisions

- **Error handling:** `anyhow` everywhere, single handler in `main`
- **Vault resolution:** `--vault` flag → `KB_VAULT` env → `~/.kb/config.toml`
- **Home dir override:** `KB_HOME` env (used in tests)
- **Domains:** top-level dirs not starting with `_` or `.`
- **No frontmatter** in most vault notes — metadata is inline bold text
- **Phase 1:** in-memory only, full index built per run
- **Phase 2:** Tantivy index at `~/.kb/index/`, SQLite dropped in favour of Tantivy

## Test Setup

```bash
cargo test                # run all tests
cargo test domains         # run only domains tests
```

Test isolation: each test copies `tests/fixtures/vault/` into a fresh `TempDir`.
Shared helpers in `tests/common/mod.rs` — `setup_vault()`, `kbase()`.

Fixture vault layout:
```
tests/fixtures/vault/
  elasticsearch/   (3 notes, includes esql/ subdir)
  lucene/          (5 notes, includes indexing/ subdir)
  rust/            (1 note)
  __templates/     (excluded)
  _logs/           (excluded)
  01-home.md       (root file, not a domain)
```

## Manual Testing Against Fixture

When developing features, test the installed binary against the fixture vault before updating tests:

```bash
# Create isolated config in /tmp
cd /tmp && rm -rf test-kbase && mkdir test-kbase

# Add fixture vault
KBASE_HOME=/tmp/test-kbase/.kbase kbase add test-vault /Users/balu/Code/kbase/tests/fixtures/vault

# Test commands
KBASE_HOME=/tmp/test-kbase/.kbase kbase domains
KBASE_HOME=/tmp/test-kbase/.kbase kbase notes --domain lucene
```

This lets you see actual output and iterate quickly without `cargo test` rebuild cycles.
