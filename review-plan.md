# Code Review Plan

## What is this

The codebase has a lot of AI-generated code. The goal is to read every file,
clean up naming, remove duplication, fix wrong comments, and make the code
feel intentional. We go bottom-up — dependencies before the files that use them
— so by the time you reach a file, everything it builds on is already clean.

Approach per file:
- Read it fully before touching anything
- Ask questions about anything unclear
- Make small, focused changes
- Run `cargo test` after each meaningful change

---

## Done

- **`src/config.rs`**
  - Renamed `CONFIG_DIR` → `DEFAULT_KB_HOME`
  - Renamed env var `KB_CONFIG_DIR` → `KB_HOME` everywhere (src + tests)
  - Fixed wrong comment on `config_path()` (it said KB_HOME pointed to `.kb` directly, which it now actually does)
  - Extracted `pub fn kb_home()` — single place for kb home resolution, used by all command files
  - Made `config_path()` pub and added doc comment
  - Moved command logic (`show`, `add_vault`, `set_active_vault`, `list_vaults`) out to `commands/config.rs` — `config.rs` is now pure data (structs, load, save, path helpers)
  - Added doc comments on `load()` and `save()`

- **`src/main.rs`**
  - Removed stale comment `// Remove ConfigAction enum entirely`
  - Added `SortBy` enum (Name, Count) replacing `sort: String` on `Domains` and `Tags` commands
  - Switched to `default_value_t = SortBy::Name` (type-safe default)
  - Shortened redundant arg doc comments (clap already shows possible values)

- **`src/commands/config.rs`**
  - Now owns the actual logic instead of being empty wrappers
  - `resolve_path()` moved here from `config.rs`

- **`src/commands/domains.rs`**
  - Updated to use `SortBy` enum instead of string comparison

- **`src/commands/tags.rs`**
  - Updated to use `SortBy` enum with exhaustive match
  - Uses `kb_home()` instead of inline env var logic

- **`src/commands/index.rs`**
  - Uses `kb_home()` instead of inline env var logic
  - Cleaned up unused `anyhow` import

- **`src/vault.rs`** (in progress)
  - Removed unused `path` field from `Domain`
  - Added struct-level doc comments on `Vault` and `Domain`

---

## Pending

### Source Files

| # | File | Status | Notes |
|---|------|--------|-------|
| 2 | `src/vault.rs` | in progress | resume here |
| 3 | `src/tags/extract.rs` | [ ] | looks clean, has inline tests |
| 4 | `src/tags/index.rs` | [ ] | `filter_by_domains` is unused — remove or use it |
| 5 | `src/tags/mod.rs` | [ ] | tiny, just re-exports |
| 8 | `src/commands/read.rs` | [ ] | looks clean |
| 10 | `src/commands/index.rs` | [ ] | mostly done, do a final read |
| 11 | `src/commands/notes.rs` | [ ] | most complex — has duplication (see below) |
| 12 | `src/commands/mod.rs` | [ ] | dispatcher, straightforward |

### Test Files

| # | File | Status | Notes |
|---|------|--------|-------|
| 14 | `tests/common/mod.rs` | [ ] | |
| 15 | `tests/config.rs` | [ ] | has local `kb()` fn duplicating `common::kb` — fix |
| 16 | `tests/domains.rs` | [ ] | |
| 17 | `tests/read.rs` | [ ] | |
| 18 | `tests/index.rs` | [ ] | |
| 19 | `tests/notes.rs` | [ ] | |

---

## Known Issues Still to Fix

**Duplication in `commands/notes.rs`**
- `read_first_heading()` is duplicated from `vault.rs`. Fix: add `Vault::note_from_path(&self, path: &str) -> Result<Note>` to `vault.rs` and delete the copy in `notes.rs`.

**Unused code**
- `filter_by_domains()` in `tags/index.rs` is never called. Decide: implement the feature that uses it or remove it.

**Test issue**
- `tests/config.rs` defines its own local `kb()` fn instead of importing from `common`. Remove it and use `common::kb`.
