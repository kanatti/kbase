# Implementation Plan

Incremental steps — each leaves the tool in a working, useful state.

## Step 1: Config
`kb config` and `kb config set vault <path>`.  
Foundation for everything. Nothing else works without it.

## Step 2: Topics + Ls
`kb topics` and `kb ls <topic>`.  
Pure filesystem, no parsing. First real navigation.

## Step 3: Note Resolution + Read
`kb read <note>`.  
Needs the note resolution logic (short name → full path). This resolver is used by almost every command after this.

## Step 4: Outline
`kb outline <note>`.  
Needs basic heading parser on a single file. First use of parsing.

## Step 5: Search
`kb search`.  
Ripgrep wrapper with topic scoping. Most useful command day-to-day.

## Step 6: Tasks
`kb tasks` and `kb task`.  
Ripgrep for finding tasks, file mutation for marking done.

## Step 7: Parser + Index
Parse every note into a `Note` struct, build the in-memory index.  
No new commands — this is the engine for everything below.

## Step 8: Link Graph
`kb links`, `kb backlinks`, `kb orphans`, `kb deadends`, `kb unresolved`.  
All powered by the index.

## Step 9: Tags
`kb tags`, `kb tag`.  
Also powered by the index.

## Step 10: Write Ops
`kb new`, `kb append`, `kb daily`.

## Step 11: Stats + Report
`kb stats`, `kb report`.  
Compose everything above.

## Step 12: Research
`kb research`.  
Capstone — composes navigation + search + tasks + links.

---

## Revised Plan (current)

Steps 1–3 complete. Steps 4–6 replaced — outline folded into `kb read --outline`,
ripgrep search skipped in favour of going straight to Tantivy.
Multi-vault support added before the index work begins.

### Completed

| Step | What | Status |
|------|------|--------|
| 1 | `kb config`, `kb config set vault` | ✅ |
| 2 | `kb topics`, `kb notes`, `--sort`, `--topic`, `--files` | ✅ |
| 3 | `kb read <path>`, `kb read --outline` | ✅ |

### Next

**Step 4 — Multi-vault config**  
Change `Config` from a single `vault` field to `default + HashMap<name, path>`.  
Add `kb vault add/list/default/remove`.  
Update vault resolution: `KB_VAULT` (path) → `--vault <name>` → config default.  
`KB_VAULT` stays as a raw path override so tests need no changes.  
See `docs/config.md`.

**Step 5 — Parser (`src/parser.rs`)**  
Parse a single note into `ParsedNote`:
- YAML frontmatter (`tags:` field)
- First `# Heading` → title
- All headings → `Vec<Heading>`
- Inline `#tags` via regex
- Wikilinks `[[target]]` / `[[target|alias]]` via regex
- Full body text for Tantivy

Uses pulldown-cmark for headings + regex for Obsidian-specific syntax.  
See `docs/parsing.md`.

**Step 6 — Index store (`src/index_store.rs`)**  
Tantivy schema + build + query. Tags and links as JSON sidecars.  
Per-vault index at `~/.kb/vaults/<name>/`.  
Wikilink resolution: path-style (`topic/note`) tried directly; bare names
tried same-topic first, then global scan; ambiguous → unresolved.  
See `docs/index.md`.

**Step 7 — `kb index` command**  
Wire parser + index store into a single rebuild command.  
Full rebuild every run (no incremental).  
Output: note count + unresolved wikilink count.

**Step 8 — `kb notes --term`**  
Query Tantivy index. Error if index missing ("run `kb index` first").  
Scoped by `--topic` when provided.  
Results ranked by BM25, displayed same as `kb notes`.

**Step 9 — Link graph commands**  
`kb links <note>`, `kb backlinks <note>`, `kb orphans`, `kb deadends`.  
All read from `links.json` — no vault scan at query time.

**Step 10 — Tags commands**  
`kb tags` (all tags + counts), `kb tag <name>` (notes with tag).  
Read from `tags.json`.

**Step 11 — Write ops**  
`kb new`, `kb append`, `kb daily`.

**Step 12 — Stats + Research**  
`kb stats`, `kb report`, `kb research`.
