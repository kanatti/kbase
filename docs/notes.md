# Notes

List and filter notes across the vault.

## List all notes

```bash
kb notes                       # all notes in the vault
```

Shows a table with vault-relative paths and titles:

```
Path                           Title
elasticsearch/esql-analysis.md ESQL Analysis
lucene/01-home.md              Lucene
lucene/search-flow.md          Search Flow Deep Dive
rust/ownership.md              ownership
```

**Title extraction:**
- Extracts the first `# Heading` from each note (scans first 20 lines)
- Falls back to filename stem if no heading found

## Filter by domain

```bash
kb notes --domain lucene       # notes in a specific domain
```

Shows only notes within the specified domain folder.

**Errors:**
- Unknown domain: `Error: Domain 'xyz' does not exist`

## Filter by tag

```bash
kb notes --tag deep-dive                    # notes with this tag
kb notes --tag wip --domain lucene          # combine tag + domain
```

**Requires:** `kb index` must be run first to build the tag index.

**Errors:**
- No index: `No tag index found. Run 'kb index' to build it first.`
- Tag not found: `No notes with tag 'xyz'.`
- No results: `No notes in domain 'lucene' with tag 'xyz'.`

## Show paths only

```bash
kb notes --files                            # all notes, paths only
kb notes --domain lucene --files            # domain filter, paths only
kb notes --tag rust --files                 # tag filter, paths only
```

Outputs only vault-relative paths, one per line (no table, no titles):

```
lucene/01-home.md
lucene/search-flow.md
lucene/codecs.md
```

Useful for piping to other tools.

## Full-text search (not yet implemented)

```bash
kb notes --term "search flow"  # NOT IMPLEMENTED YET
```

Shows error: `--term search is not yet implemented`
