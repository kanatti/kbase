# Notes

List and filter notes across the vault.

## List all notes

```bash
kbase notes                       # all notes in the vault
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
kbase notes --domain lucene       # notes in a specific domain
```

Shows only notes within the specified domain folder.

**Errors:**
- Unknown domain: `Error: Domain 'xyz' does not exist`

## Filter by tag

```bash
kbase notes --tag deep-dive                    # notes with this tag
kbase notes --tag wip --domain lucene          # combine tag + domain
```

**Requires:** `kbase index` must be run first to build the tag index.

**Errors:**
- No index: `No tag index found. Run 'kbase index' to build it first.`
- Tag not found: `No notes with tag 'xyz'.`
- No results: `No notes in domain 'lucene' with tag 'xyz'.`

## Show paths only

```bash
kbase notes --files                            # all notes, paths only
kbase notes --domain lucene --files            # domain filter, paths only
kbase notes --tag rust --files                 # tag filter, paths only
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
kbase notes --term "search flow"  # NOT IMPLEMENTED YET
```

Shows error: `--term search is not yet implemented`
