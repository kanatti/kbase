# kbase — Knowledge Base CLI Specification

Personal knowledge base CLI for navigating, searching, and maintaining
`~/Documents/kanatti-notes`. Built to work from any directory — inside a code
repo, a pi session, or standalone in the terminal.

---

## Table of Contents

1. [Goals](#goals)
2. [Vault Conventions](#vault-conventions)
3. [Architecture](#architecture)
4. [Index Design](#index-design)
5. [Command Reference](#command-reference)
   - [Navigation](#navigation)
   - [Search](#search)
   - [Link Graph](#link-graph)
   - [Tags](#tags)
   - [Tasks](#tasks)
   - [Read & Write](#read--write)
   - [Research Mode](#research-mode)
   - [Reports & Stats](#reports--stats)
   - [Index Management](#index-management)
6. [Pi Skill: kbase-notes](#pi-skill-kbase-notes)
7. [Installation](#installation)
8. [Phase 2: SQLite FTS5](#phase-2-sqlite-fts5)

---

## Goals

- Navigate 600+ markdown notes from the terminal without opening Obsidian
- Search across the full vault or scoped to one or more domains
- Map the link graph: backlinks, orphans, broken links, clusters
- Track tasks across the vault
- Support multi-domain research sessions (e.g. lucene + datafusion + arrow together)
- Be a first-class tool for the pi agent — Claude can navigate and update notes
  during any coding session
- Stay simple: text files in, text output out, no daemons, no servers

---

## Vault Conventions

`kbase` is built around `~/Documents/kanatti-notes`. Understanding its structure
is required to implement correctly.

### Directory Layout

```
kanatti-notes/
├── 01-home.md              # Root index / navigation guide
├── 02-lists.md             # Curated lists (books, videos, links)
├── 03-tasks.md             # All committed tasks (this week / backlog)
├── 04-thoughts.md          # Reflections and observations
├── 05-ideas.md             # Uncommitted ideas
├── 11-archives.md          # Completed work, milestones
├── CLAUDE.md               # Agent instructions (Claude Code)
├── AGENTS.md               # Agent instructions (other agents)
│
├── _logs/                  # Daily planning journals (monthly files)
│   ├── guidance.md
│   ├── 2026-02.md
│   └── 2025/
├── _planning/              # Long-term goals, yearly plans, reviews
│   ├── guidance.md
│   ├── longterm.md
│   └── 2026.md
│
├── __templates/            # Note templates (excluded from search by default)
├── __attachments/          # Binary attachments (excluded from search)
├── __canvas/               # Obsidian canvas files (excluded from search)
│
├── elasticsearch/          # Domain folder
│   ├── 01-home.md          # Domain index
│   ├── 02-logs.md          # Domain work log
│   ├── 03-task-board.md    # Domain tasks
│   ├── 04-archive.md       # Completed work in this domain
│   └── esql-analysis.md    # Content notes
├── lucene/
├── datafusion/
├── rust/
└── ...                     # ~50 domain folders total
```

### Domain Folder Convention

Most domain folders follow this pattern:
- `01-home.md` — overview, goals, current focus, links to key docs
- `02-logs.md` — chronological work log
- `03-task-board.md` — tasks specific to this domain
- `04-archive.md` — completed phases/tasks

Content notes are flat files in the domain folder, named with kebab-case.

### Wikilink Format

Obsidian `[[wikilink]]` format is used throughout:

```
[[note-name]]                    # short form, resolved by filename
[[domain/note-name]]              # full path form
[[note-name|display text]]       # aliased link
[[note-name#Section Heading]]    # link to heading
```

Resolution rules (same as Obsidian):
1. Exact path match from vault root: `domain/note-name.md`
2. Filename match anywhere in vault: `note-name.md`
3. Unresolved if neither found

### Task Format

Standard markdown checkboxes (Obsidian-compatible):

```markdown
- [ ] incomplete task
- [x] completed task
- [-] cancelled task
- [?] question/maybe
```

Tasks appear in regular content, not just dedicated task files.

### Tags

Inline `#tag` format used sparsely. No strict taxonomy yet. Examples seen:
`#deep-dive`, `#wip`, `#learning`, `#reference`

### Frontmatter

Most notes do NOT have YAML frontmatter — they use bold headers for metadata:

```markdown
**Status:** Learning - Phase 2B
**Alignment:** 🟢 Target employer
```

Some notes may have frontmatter. `kbase` should handle both patterns.

### Excluded Paths

By default, these paths are excluded from all operations:
- `__templates/`
- `__attachments/`
- `__canvas/`
- `_logs/` (excluded from search by default, included with `--logs` flag)

---

## Architecture

### Language & Dependencies

**Python 3.10+** — available everywhere, no pip installs needed for Phase 1.

Phase 1 (filesystem-based):
- `pathlib` — file/dir traversal
- `re` — wikilink, task, tag, heading parsing
- `subprocess` — ripgrep for full-text search
- `json` — structured output
- `sqlite3` — stdlib, used in Phase 2

Phase 1 requires `rg` (ripgrep) for full-text search. Falls back to `grep -r`
if ripgrep is not available with a performance warning.

### File Locations

```
~/Documents/kanatti-notes/      # KBASE_ROOT — the vault
~/Documents/kanatti-notes/.scripts/kbase   # the script itself
~/.kbase/                          # kbase state dir
~/.kbase/index.db                  # SQLite index (Phase 2)
~/.kbase/config.json               # config overrides
~/bin/kbase                        # symlink to .scripts/kbase
```

### Config (`~/.kbase/config.json`)

```json
{
  "vault": "~/Documents/kanatti-notes",
  "editor": "code",
  "exclude": ["__templates", "__attachments", "__canvas"],
  "exclude_search": ["_logs", "_planning"],
  "ripgrep": true
}
```

Config is optional. All values have defaults. Can be overridden per-call with
flags.

### Output Modes

All commands support:
- **Default**: human-readable, colourised terminal output
- `--json`: machine-readable JSON (for pi agent use and scripting)
- `--quiet` / `-q`: minimal output, paths only

The pi skill always calls `kbase` with `--json` and parses the output.

---

## Index Design

### Phase 1: In-Memory (per-call)

For Phase 1, the link graph and tag index are built on-the-fly per command
invocation. With 658 files this takes <200ms — acceptable for interactive use.

**Link graph build** (used by `links`, `backlinks`, `orphans`, `deadends`,
`unresolved`):

```python
def build_link_graph(vault: Path) -> dict:
    """
    Returns:
      {
        "forward": { "domain/note": ["domain/other", "other-domain/note"] },
        "reverse": { "domain/note": ["domain/source"] },
        "all_notes": set of all note paths relative to vault root,
        "unresolved": { "domain/note": ["broken-link-name"] }
      }
    """
```

Wikilink resolution: given `[[note-name]]`, try:
1. `{note-name}.md` relative to the source file's folder
2. `**/{note-name}.md` anywhere in vault (first match wins)
3. If not found → unresolved

### Phase 2: SQLite FTS5

See [Phase 2](#phase-2-sqlite-fts5) section.

---

## Command Reference

### Global Flags

These flags work on all commands:

```
--json          output as JSON instead of human-readable text
--quiet, -q     minimal output (paths only)
--vault PATH    override vault path (default: ~/Documents/kanatti-notes)
--logs          include _logs/ in results (excluded by default)
--planning      include _planning/ in results (excluded by default)
--help, -h      show help
```

---

### Navigation

#### `kbase domains`

List all domain folders with note counts.

```
kbase domains
kbase domains --sort count       # sort by note count descending
kbase domains --sort name        # sort alphabetically (default)
```

Output:
```
elasticsearch      18 notes
lucene             22 notes
datafusion          9 notes
rust               14 notes
...
```

Implementation: `os.scandir(vault)`, filter for directories not starting with
`_` or `__`, count `.md` files in each.

---

#### `kbase ls <domain>`

List notes in a domain folder with their first heading (or filename if no
heading).

```
kbase ls lucene
kbase ls lucene elasticsearch          # list notes across multiple domains
kbase ls lucene --files                # filenames only, no heading preview
```

Output:
```
lucene/
  01-home.md              Lucene
  binary-doc-values.md    Binary Doc Values
  codecs-deep-dive.md     Codecs Deep Dive
  doc-values.md           Doc Values
  postings-format.md      Postings Format
  search-flow-deep-dive.md  Deep Dive: TermQuery + TopScoreDocCollector
  ...
```

Implementation: list `.md` files in the domain dir, read first `# Heading` from
each file (first 5 lines only, fast).

---

#### `kbase outline <note>`

Show the heading structure of a note as an indented tree.

```
kbase outline lucene/search-flow-deep-dive
kbase outline esql-analysis               # resolves by filename
```

Output:
```
# Deep Dive: TermQuery + TopScoreDocCollector Search Flow
  ## Setup: The Index Structure
  ## Usage
  ## Phase 1: IndexSearcher.search()
    ### Step 1: createWeight()
    ### Step 2: BulkScorer
  ## Phase 2: Scoring
```

Implementation: regex `^(#{1,6})\s+(.+)` on each line, indent by heading level.

---

#### `kbase random [domain]`

Print a random note from the vault or a specific domain. Useful for spaced
review — surfacing notes you forgot about.

```
kbase random
kbase random lucene
kbase random --read             # also print the note content
```

Output:
```
lucene/gcd-compression.md   GCD Compression
```

---

### Search

#### `kbase search <query>`

Full-text search across the vault using ripgrep.

```
kbase search "BKD tree"
kbase search "BKD tree" --domain lucene elasticsearch    # scoped to domains
kbase search "BKD tree" --matches                  # show matching lines
kbase search "BKD tree" --matches --context 2      # +2 lines of context
kbase search "phase:2" --matches                   # works for any pattern
kbase search "TODO\|FIXME" --matches               # regex
```

Output (default — file list):
```
lucene/binary-doc-values.md
lucene/doc-values.md
elasticsearch/esql-analysis.md
```

Output (`--matches`):
```
lucene/binary-doc-values.md
  Line 14: The BKD tree is a k-d tree variant used for numeric range queries.
  Line 47: BKD tree nodes are stored in a single packed byte array...

elasticsearch/esql-analysis.md
  Line 203: Uses BKD-backed PointsValues for numeric fields
```

Flags:
```
--domain DOMAIN [DOMAIN...]   limit search to these domain folders
--matches                       show matching lines (like rg --no-heading)
--context N, -C N              lines of context around matches (implies --matches)
--case, -s                      case-sensitive (default: case-insensitive)
--files-only                    print filenames only (no counts)
--total                         print match count only
```

Implementation:
```python
def search(query, domains=None, matches=False, context=0, case=False):
    paths = [vault/t for t in domains] if domains else [vault]
    cmd = ["rg", "--glob=*.md", "-i" if not case else ""]
    if not matches:
        cmd += ["-l"]
    if context:
        cmd += [f"-C{context}"]
    for p in paths:
        cmd += [query, str(p)]
    # Run per-path and merge results
```

When multiple domains are specified, run ripgrep once per domain and deduplicate
results. Present results grouped by domain.

---

### Link Graph

All link graph commands build the in-memory graph on-the-fly. Graph build time
is ~100-150ms for 658 files. Results are cached in the process for chained
operations.

#### `kbase links <note>`

List all wikilinks going OUT of a note (outgoing links).

```
kbase links esql-analysis
kbase links elasticsearch/esql-analysis    # full path also works
kbase links esql-analysis --resolved       # only resolved links
kbase links esql-analysis --unresolved     # only broken links
```

Output:
```
Outgoing links from elasticsearch/esql-analysis.md (8 links):
  ✓ elasticsearch/01-home             elasticsearch/01-home.md
  ✓ esql-learning-plan                elasticsearch/esql-learning-plan.md
  ✓ esql-query-flow                   elasticsearch/esql-query-flow.md
  ✗ esql-optimizer                    (unresolved — note doesn't exist yet)
  ✗ esql-execution                    (unresolved — note doesn't exist yet)
```

---

#### `kbase backlinks <note>`

List all notes that link TO this note (incoming links / backlinks).

```
kbase backlinks esql-analysis
kbase backlinks esql-analysis --counts     # include how many links per source
```

Output:
```
Backlinks to elasticsearch/esql-analysis.md (3 notes):
  elasticsearch/01-home.md             (1 link)
  elasticsearch/03-task-board.md       (2 links)
  elasticsearch/esql-learning-plan.md  (1 link)
```

---

#### `kbase orphans`

List notes with NO incoming links — nothing in the vault points to them.
These are forgotten or isolated notes.

```
kbase orphans
kbase orphans --domain lucene               # only check within lucene domain
kbase orphans --total                 # count only
```

Output:
```
Orphan notes (no incoming links): 23

lucene/rank-table.md
lucene/leading-vs-validating-queries.md
datafusion/arrow-ipc.md
rust/unsafe-patterns.md
...
```

---

#### `kbase deadends`

List notes with NO outgoing links — they link to nothing else.

```
kbase deadends
kbase deadends --domain elasticsearch
```

---

#### `kbase unresolved`

List all broken wikilinks across the vault — links that point to notes that
don't exist.

```
kbase unresolved
kbase unresolved --domain elasticsearch          # scoped to a domain
kbase unresolved --verbose                 # show source file for each broken link
```

Output:
```
Unresolved wikilinks: 7

esql-optimizer             referenced in elasticsearch/esql-analysis.md, elasticsearch/03-task-board.md
esql-execution             referenced in elasticsearch/03-task-board.md
arrow-ipc-format           referenced in arrow/01-home.md
```

These are often notes you planned to write but haven't yet — useful as a
writing backlog.

---

### Tags

#### `kbase tags`

List all `#tags` across the vault with counts.

```
kbase tags
kbase tags --sort count               # sort by frequency (default: name)
kbase tags --domain lucene elasticsearch    # scoped to domains
```

Output:
```
#deep-dive    12
#wip           8
#learning      7
#reference     5
```

Implementation: `rg '#[a-zA-Z][a-zA-Z0-9_-]*'` with `-o` flag (output only
matches), collect and count.

---

#### `kbase tag <name>`

List all notes that contain a specific tag.

```
kbase tag deep-dive                # # prefix optional
kbase tag #wip --verbose          # show matching line context
```

Output:
```
Notes tagged #deep-dive (12):
  lucene/search-flow-deep-dive.md
  lucene/codecs-deep-dive.md
  elasticsearch/esql-query-flow.md
  ...
```

---

### Tasks

#### `kbase tasks`

List tasks across the vault.

```
kbase tasks                          # all incomplete tasks in vault
kbase tasks --done                   # completed tasks
kbase tasks --all                    # both complete and incomplete
kbase tasks --domain elasticsearch         # scoped to a domain
kbase tasks --domain elasticsearch --file 03-task-board   # scoped to a file
kbase tasks --verbose                # grouped by file with line numbers
kbase tasks --total                  # count only
```

Output (default):
```
elasticsearch/03-task-board.md:12   [ ] Complete esql-analysis documentation
elasticsearch/03-task-board.md:14   [ ] Create esql-optimizer document
elasticsearch/03-task-board.md:15   [ ] Create esql-execution document
lucene/01-home.md:8                 [ ] LongPoint and BKD Trees
...
```

Output (`--verbose`):
```
elasticsearch/03-task-board.md
  Line 12: [ ] Complete esql-analysis documentation
  Line 14: [ ] Create esql-optimizer document
  Line 15: [ ] Create esql-execution document

lucene/01-home.md
  Line  8: [ ] LongPoint and BKD Trees
```

Implementation: `rg '- \[[ x\-?]\]'` across vault, parse status character,
format output.

---

#### `kbase task done <ref>`

Mark a specific task as done. Ref format is `file:line`.

```
kbase task done elasticsearch/03-task-board:12
kbase task done 03-task-board:12              # filename resolved
kbase task done 03-task-board:12 --status=-   # set to [-] cancelled
```

Implementation: read the file, replace `[ ]` with `[x]` on the given line,
write back. Check the line actually contains a task before modifying.

---

### Read & Write

#### `kbase read <note>`

Print note content to stdout.

```
kbase read esql-analysis
kbase read elasticsearch/esql-analysis     # full path also works
kbase read esql-analysis --outline         # print outline only (headings)
```

Note resolution:
1. Try as literal path from vault root: `elasticsearch/esql-analysis.md`
2. Try with `.md` extension appended
3. Try filename match anywhere in vault (first match)
4. Error if not found with suggestions ("did you mean: ...?")

---

#### `kbase open <note>`

Open a note in the configured editor (default: `$EDITOR`, fallback: `code`).

```
kbase open esql-analysis
kbase open esql-analysis --obsidian      # open via obsidian:// URI if available
```

---

#### `kbase append <note> <content>`

Append content to an existing note without opening it.

```
kbase append esql-analysis "## New Finding\n\nContent here"
kbase append esql-analysis --file patch.md     # append from file
```

Appends with a preceding blank line if the note doesn't end with one.

---

#### `kbase new <domain> <name>`

Create a new note in a domain folder.

```
kbase new lucene jump-tables-deep-dive
kbase new lucene jump-tables --content "# Jump Tables\n\n"
kbase new lucene jump-tables --template default
kbase new lucene jump-tables --open          # open after creating
```

Default content if no `--content` or `--template`:
```markdown
# {Title Case of Name}

```

Templates are read from `__templates/` in the vault. Template name maps to
`__templates/{name}.md`.

---

#### `kbase daily`

Show the path to today's daily log file.

```
kbase daily                    # print path
kbase daily --read             # print content
kbase daily --append "- [ ] Review BKD tree notes"
```

Daily log path resolution: `_logs/YYYY-MM.md` based on current date.

---

### Research Mode

#### `kbase research <domain> [domain2 ...]`

Multi-domain exploration mode. Combines listing and search across a set of
related domains. Designed for deep-dive sessions that span multiple areas.

```
kbase research lucene datafusion arrow
kbase research elasticsearch lucene opensearch
```

Output:

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Research Session: lucene • datafusion • arrow
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

DOMAINS
  lucene        22 notes    7 open tasks
  datafusion     9 notes    3 open tasks
  arrow          5 notes    1 open task

HOME NOTES
  lucene/01-home.md
  datafusion/01-home.md
  arrow/01-home.md

OPEN TASKS (across all domains)
  lucene/01-home.md:8                  [ ] LongPoint and BKD Trees
  datafusion/01-home.md:12             [ ] Read DataFusion query optimizer source
  ...

CROSS-LINKS (links between these domains)
  datafusion/query-execution.md  →  arrow/columnar-format.md
  arrow/01-home.md               →  lucene/binary-doc-values.md

RECENT NOTES (last modified)
  lucene/search-flow-deep-dive.md       2 days ago
  datafusion/physical-plan.md           5 days ago
  arrow/ipc-format.md                   2 weeks ago

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Type a search query to search across these domains, or Ctrl+C to exit.
> 
```

After printing the overview, drops into an interactive search loop scoped to
the selected domains. Each query runs `kbase search <query> --domain <domains>` and prints
results. Ctrl+C exits.

If `--json` flag is passed, outputs the overview as JSON and exits immediately
(no interactive mode). This is what the pi agent uses.

---

### Reports & Stats

#### `kbase stats`

Vault statistics summary.

```
kbase stats
kbase stats --domain lucene           # stats for a single domain
```

Output:
```
Vault: ~/Documents/kanatti-notes
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Notes              658
Domains              52
Open tasks          47
Total words    ~94,000
Orphan notes        23
Unresolved links     7
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Largest domains:
  lucene             22 notes
  elasticsearch      18 notes
  rust               14 notes
  delta              29 notes
  gen-ai             11 notes
```

---

#### `kbase report`

Generate a full vault health report. Combines orphans, dead ends, unresolved
links, and idle notes into one output.

```
kbase report
kbase report --output report.md    # save to a markdown file
```

Output:
```
Vault Health Report — 2026-02-16
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

UNRESOLVED LINKS (7)
  These wikilinks point to notes that don't exist.
  esql-optimizer    ← elasticsearch/esql-analysis.md, elasticsearch/03-task-board.md
  esql-execution    ← elasticsearch/03-task-board.md
  ...

ORPHAN NOTES (23)
  Notes that nothing links to. Possibly forgotten.
  lucene/rank-table.md
  lucene/leading-vs-validating-queries.md
  ...

DEAD-END NOTES (15)
  Notes with no outgoing links. Possibly stubs.
  rust/unsafe-patterns.md
  ...

IDLE NOTES (not modified in 90+ days, 34)
  Consider reviewing or archiving.
  delta/yfinance-code-reading-plan.md       180 days ago
  ...
```

---

### Index Management

#### `kbase index`

Build or refresh the SQLite FTS5 index. Phase 2 feature — no-op in Phase 1
(prints "Index not implemented in Phase 1").

```
kbase index               # build or refresh
kbase index --rebuild     # force full rebuild
kbase index --status      # show index state (last built, note count)
```

---

## Pi Skill: kbase-notes

### Location

`~/.pi/agent/skills/kbase-notes/` — global skill, available in every pi session.

### What the skill does

Teaches the pi agent how to navigate and work with `kanatti-notes` during any
session. When the agent is:
- Exploring code in `~/Code/elasticsearch` → can pull relevant notes as context
- Asked to "research how datafusion handles memory" → knows to use `kbase research`
- Writing new findings → knows to `kbase append` or `kbase new`
- Asked about tasks → knows to use `kbase tasks`

### Skill file: `SKILL.md`

```markdown
---
name: kbase-notes
description: Access and manage the kanatti-notes knowledge base. Use when the user asks to search, read, update, or navigate personal notes; when researching a technical domain and need context from prior notes; or when working in a code repo that has a related notes domain.
---

# kbase-notes — Knowledge Base Skill

Personal knowledge base at ~/Documents/kanatti-notes. 658+ markdown notes
organized by domain. Use `kbase` CLI for all operations.

## Key Conventions

- Domains = top-level folders: elasticsearch, lucene, datafusion, rust, etc.
- Each domain has 01-home.md (overview), 03-task-board.md (tasks)
- Notes use [[wikilink]] format for cross-references
- Tasks: `- [ ]` incomplete, `- [x]` done
- No YAML frontmatter in most notes — metadata is inline bold text

## Workflows

### Before starting research on a domain
1. `kbase read <domain>/01-home.md` — get the domain overview and current focus
2. `kbase ls <domain>` — see all notes in the domain
3. `kbase tasks --domain <domain>` — see open tasks

### When exploring a code repo
Match the repo name to a domain:
- ~/Code/elasticsearch → domain: elasticsearch
- ~/Code/lucene → domain: lucene
- ~/Code/datafusion → domain: datafusion
- ~/Code/arrow-rs → domain: arrow
Run `kbase read <domain>/01-home.md` to get context before diving into code.

### Cross-domain research
`kbase research lucene datafusion arrow` — shows overview of all three domains,
cross-links between them, and drops into interactive search.
Use `--json` for machine-readable output.

### Finding relevant notes
`kbase search "<query>" --matches` — full-text with context
`kbase search "<query>" --domain <domain>` — scoped to a domain
`kbase backlinks <note>` — find related notes that link here

### Adding new knowledge
`kbase append <note> "## New Finding\n\ncontent"` — add to existing note
`kbase new <domain> <name>` — create new note

## Common Commands

\`\`\`bash
kbase domains                                # all domains with counts
kbase ls lucene                             # list notes in lucene
kbase read lucene/search-flow-deep-dive     # read a specific note
kbase search "columnar format" --matches    # full-text search
kbase search "hash join" --domain datafusion arrow --matches
kbase research lucene datafusion --json     # research session overview
kbase tasks --domain elasticsearch                # open tasks in a domain
kbase backlinks esql-analysis               # what links to this note
kbase orphans                               # forgotten notes
kbase report                                # full vault health check
\`\`\`
```

---

## Installation

### Step 1: Create the script

```bash
mkdir -p ~/Documents/kanatti-notes/.scripts
# Write kbase script to ~/Documents/kanatti-notes/.scripts/kbase
chmod +x ~/Documents/kanatti-notes/.scripts/kbase
```

### Step 2: Add to PATH

```bash
mkdir -p ~/bin
ln -s ~/Documents/kanatti-notes/.scripts/kbase ~/bin/kbase
# Ensure ~/bin is in PATH — add to ~/.zshrc if not:
# export PATH="$HOME/bin:$PATH"
```

### Step 3: Verify ripgrep

```bash
which rg || brew install ripgrep
```

### Step 4: Install the pi skill

```bash
mkdir -p ~/.pi/agent/skills/kbase-notes
# Write SKILL.md to ~/.pi/agent/skills/kbase-notes/SKILL.md
```

### Step 5: Cross-repo symlinks (optional)

For quick access from code repos:

```bash
for domain in elasticsearch lucene datafusion arrow rust; do
  ln -s ~/Documents/kanatti-notes/$domain ~/Code/$domain/.notes 2>/dev/null
done
```

Then from `~/Code/elasticsearch` you can do:
```bash
kbase read .notes/01-home.md
# or just:
kbase read elasticsearch/01-home      # always works from anywhere
```

---

## Phase 2: SQLite FTS5

When the Phase 1 filesystem approach gets slow or you need ranked results:

### Schema

```sql
-- Full-text search index
CREATE VIRTUAL TABLE notes_fts USING fts5(
  path,          -- relative path from vault root
  title,         -- first # heading
  content,       -- full note content
  tokenize = 'porter unicode61'
);

-- Structured metadata (separate from FTS)
CREATE TABLE notes (
  path TEXT PRIMARY KEY,
  domain TEXT,
  title TEXT,
  word_count INTEGER,
  modified_at INTEGER,    -- unix timestamp
  created_at INTEGER
);

CREATE TABLE links (
  source_path TEXT,
  target_path TEXT,       -- resolved, or NULL if unresolved
  target_raw TEXT,        -- raw wikilink text
  is_resolved INTEGER
);

CREATE TABLE tags (
  path TEXT,
  tag TEXT
);

CREATE TABLE tasks (
  path TEXT,
  line INTEGER,
  status TEXT,            -- ' ', 'x', '-', '?'
  content TEXT
);

CREATE TABLE headings (
  path TEXT,
  level INTEGER,
  text TEXT,
  line INTEGER
);
```

### Index Build

```bash
kbase index           # build from scratch or update changed files
kbase index --rebuild # force full rebuild
```

Incremental update: compare `mtime` of each `.md` file against `notes.modified_at`.
Only re-index changed files.

### Benefits over Phase 1

| Feature | Phase 1 | Phase 2 |
|---|---|---|
| Full-text search | ripgrep (~30ms) | FTS5 ranked (~10ms) |
| Backlinks | scan all files (~150ms) | index lookup (~1ms) |
| Tag listing | ripgrep scan | index lookup |
| Task listing | ripgrep scan | index lookup |
| Search ranking | none (filename order) | TF-IDF via FTS5 |
| Frontmatter queries | not supported | `WHERE property = value` |
| Cross-note analytics | slow | fast JOINs |

The Phase 2 index is purely additive — all Phase 1 commands still work without
the index. The index is auto-used when present.

---

## Implementation Order

Build in this order:

1. **Navigation** (`domains`, `ls`, `outline`, `random`) — pure filesystem, no deps
2. **Read/Write** (`read`, `open`, `append`, `new`, `daily`) — filesystem
3. **Search** (`search`) — ripgrep integration
4. **Tasks** (`tasks`, `task done`) — ripgrep + file edit
5. **Tags** (`tags`, `tag`) — ripgrep
6. **Link Graph** (`links`, `backlinks`, `orphans`, `deadends`, `unresolved`) — in-memory graph
7. **Research** (`research`) — compose commands 1-6
8. **Reports** (`stats`, `report`) — compose all above
9. **Pi Skill** (`~/.pi/agent/skills/kbase-notes/SKILL.md`)
10. **Phase 2 index** (`kbase index`) — SQLite FTS5

Each step is independently useful. Stop at any point.
