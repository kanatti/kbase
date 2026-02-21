# Implementation Plan

Incremental steps — each leaves the tool in a working, useful state.

## Step 1: Config
`kbase config` and `kbase config set vault <path>`.  
Foundation for everything. Nothing else works without it.

## Step 2: Domains + Ls
`kbase domains` and `kbase ls <domain>`.  
Pure filesystem, no parsing. First real navigation.

## Step 3: Note Resolution + Read
`kbase read <note>`.  
Needs the note resolution logic (short name → full path). This resolver is used by almost every command after this.

## Step 4: Outline
`kbase outline <note>`.  
Needs basic heading parser on a single file. First use of parsing.

## Step 5: Search
`kbase search`.  
Ripgrep wrapper with domain scoping. Most useful command day-to-day.

## Step 6: Tasks
`kbase tasks` and `kbase task`.  
Ripgrep for finding tasks, file mutation for marking done.

## Step 7: Parser + Index
Parse every note into a `Note` struct, build the in-memory index.  
No new commands — this is the engine for everything below.

## Step 8: Link Graph
`kbase links`, `kbase backlinks`, `kbase orphans`, `kbase deadends`, `kbase unresolved`.  
All powered by the index.

## Step 9: Tags
`kbase tags`, `kbase tag`.  
Also powered by the index.

## Step 10: Write Ops
`kbase new`, `kbase append`, `kbase daily`.

## Step 11: Stats + Report
`kbase stats`, `kbase report`.  
Compose everything above.

## Step 12: Research
`kbase research`.  
Capstone — composes navigation + search + tasks + links.

---

## Revised Plan (current)

Steps 1–3 complete. Steps 4–6 replaced — outline folded into `kbase read --outline`,
ripgrep search skipped in favour of going straight to Tantivy.
Multi-vault support added before the index work begins.

### Completed

| Step | What | Status |
|------|------|--------|
| 1 | `kbase config`, vault management | ✅ |
| 2 | `kbase domains`, `kbase notes`, `--sort`, `--domain`, `--files` | ✅ |
| 3 | `kbase read <path>`, `kbase read --outline` | ✅ |
| 4 | Multi-vault: `kbase add`, `kbase use`, `kbase vaults` | ✅ |
| 9a | `kbase tags`, `kbase notes --tag` (tag filtering) | ✅ |

### Next

**Step 4 — Multi-vault config** ✅ **DONE**  
Changed `Config` to use `active_vault + HashMap<name, VaultConfig>`.  
Added `kbase add <name> <path>`, `kbase use <name>`, `kbase vaults`.  
First vault added becomes active automatically.

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
Per-vault index at `~/.kbase/vaults/<name>/`.  
Wikilink resolution: path-style (`domain/note`) tried directly; bare names
tried same-domain first, then global scan; ambiguous → unresolved.  
See `docs/index.md`.

**Step 7 — `kbase index` command**  
Wire parser + index store into a single rebuild command.  
Full rebuild every run (no incremental).  
Output: note count + unresolved wikilink count.

**Step 8 — `kbase notes --term`**  
Query Tantivy index. Error if index missing ("run `kbase index` first").  
Scoped by `--domain` when provided.  
Results ranked by BM25, displayed same as `kbase notes`.

**Step 9 — Link graph commands**  
`kbase links <note>`, `kbase backlinks <note>`, `kbase orphans`, `kbase deadends`.  
All read from `links.json` — no vault scan at query time.

**Step 10 — Tags commands** ✅ **PARTIALLY DONE**  
`kbase tags` (all tags + counts), `kbase notes --tag <name>` (filter by tag).  
Built tag index with `kbase index`. Tag-first filtering for efficiency.

**Step 11 — Write ops**  
`kbase new`, `kbase append`, `kbase daily`.

**Step 12 — Stats + Research**  
`kbase stats`, `kbase report`, `kbase research`.
