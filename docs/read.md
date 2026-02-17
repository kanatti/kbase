# kb read

Read a note's content to stdout.

---

## Usage

```bash
kb read <path>            # print raw markdown content
kb read <path> --outline  # print heading structure only
```

## Path Format

Always a path relative to the vault root, with `.md` extension:

```bash
kb read lucene/search-flow.md      # note inside a topic
kb read 01-home.md                 # root-level note (no topic prefix)
```

The path format is the same as `kb notes` output — copy-paste directly:

```bash
kb notes --topic lucene
  lucene/01-home.md        Lucene
  lucene/search-flow.md    Search Flow Deep Dive

kb read lucene/search-flow.md
```

No short names, no fuzzy resolution. Full path, always unambiguous.

---

## Output

### Default — raw markdown

Raw file content dumped to stdout. Nothing stripped or reformatted.

```bash
kb read lucene/search-flow.md
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
kb read lucene/search-flow.md --outline
```

```
# Search Flow Deep Dive
  ## Phase 1: IndexSearcher.search()
    ### Step 1: createWeight()
    ### Step 2: BulkScorer
  ## Phase 2: Scoring
```

Implementation: scan lines for `^#{1,6} `, indent by `(level - 1) * 2` spaces.

---

## Error Handling

If the path does not exist relative to the vault root:

```
Error: note not found: lucene/nonexistent.md
```

No suggestions. Use `kb notes --topic lucene` to find the right path.

---

## In Code

```rust
fn cmd_read(vault: &Vault, path: &str, outline: bool) -> Result<()> {
    let full_path = vault.root.join(path);

    if !full_path.exists() {
        bail!("note not found: {}", path);
    }

    let content = fs::read_to_string(&full_path)?;

    if outline {
        print_outline(&content);
    } else {
        print!("{}", content);
    }

    Ok(())
}
```
