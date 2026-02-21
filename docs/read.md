# Read

Read a note's content to stdout.

## Usage

```bash
kbase read <path>            # print raw markdown content
kbase read <path> --outline  # print heading structure only
```

## Path Format

Always a path relative to the vault root, with `.md` extension:

```bash
kbase read lucene/search-flow.md      # note inside a domain
kbase read 01-home.md                 # root-level note (no domain prefix)
```

The path format is the same as `kbase notes` output — copy-paste directly:

```bash
kbase notes --domain lucene
  lucene/01-home.md        Lucene
  lucene/search-flow.md    Search Flow Deep Dive

kbase read lucene/search-flow.md
```

No short names, no fuzzy resolution. Full path, always unambiguous.

## Output

### Default — raw markdown

Raw file content dumped to stdout. Nothing stripped or reformatted.

```bash
kbase read lucene/search-flow.md
```

```
# Search Flow Deep Dive

How TermQuery flows through IndexSearcher, Weight, Scorer and into TopDocs.

## Phase 1: IndexSearcher.search()
...
```

Pipe through `bat` or `glow` for rendered output if needed.

### `--outline` — headings only

Prints the heading tree, indented by level:

```bash
kbase read lucene/search-flow.md --outline
```

```
# Search Flow Deep Dive
  ## Phase 1: IndexSearcher.search()
    ### Step 1: createWeight()
    ### Step 2: BulkScorer
  ## Phase 2: Scoring
```

Heading detection: lines starting with 1-6 `#` followed by a space.
Indentation: `(level - 1) * 2` spaces.

## Error Handling

If the path does not exist relative to the vault root:

```
Error: note not found: lucene/nonexistent.md
```

No suggestions. Use `kbase notes --domain lucene` to find the right path.
