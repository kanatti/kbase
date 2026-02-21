# Tags

Inline `#tags` for categorizing and discovering notes across your vault.

## Usage

```bash
kbase tags                                      # list all tags
kbase tags --sort count                         # sort by usage frequency
kbase notes --tag deep-dive                     # filter notes by tag
kbase notes --tag wip --domain lucene           # combine tag + domain filter
kbase index                                     # build tag index
kbase index --only tags                         # build only tag index
```

## Tag Format

Hash-prefixed tags in note content:

```markdown
# BKD Trees

This is a #deep-dive into Lucene's #indexing architecture.

The #performance characteristics are...
```

**Rules:**
- Alphanumeric + underscore/hyphen: `#deep-dive`, `#rust_lang`, `#v2024`
- Must start with letter or number: `#tag123`, `#2024abc`
- Pure numbers ignored: `#123`, `#20298` (treated as issue/PR references)
- Code blocks (```) are skipped during extraction

## Index Storage

Tag index is stored as JSON at:

```
~/.kbase/<vault-name>/tags.json
```

Format maps tag names to note paths:

```json
{
  "deep-dive": [
    "lucene/search-flow.md",
    "elasticsearch/esql-analysis.md"
  ],
  "wip": [
    "lucene/codec-study.md"
  ]
}
```

## Commands

### `kbase tags`

List all tags with note counts.

```bash
kbase tags                          # alphabetical (default)
kbase tags --sort name              # alphabetical (explicit)
kbase tags --sort count             # by usage frequency
```

Output:

```
Tag          Notes
deep-dive    2
performance  1
wip          1
```

### `kbase notes --tag`

Filter notes by tag. Requires tag index to be built first.

```bash
kbase notes --tag deep-dive                      # all notes with this tag
kbase notes --tag deep-dive --domain lucene      # tag + domain filter
kbase notes --tag wip --files                    # show paths only
```

Output matches `kbase notes` format (table or paths).

### `kbase index`

Build tag index by scanning vault content.

```bash
kbase index                         # build all indexes (tags, links, search)
kbase index --only tags             # build only tag index
```

## Error Messages

**Missing index:**
```bash
$ kbase notes --tag deep-dive
No tag index found. Run `kbase index` to build it first.
```

**Tag not found:**
```bash
$ kbase notes --tag nonexistent
No notes with tag 'nonexistent'.
```

**Tag + domain no results:**
```bash
$ kbase notes --tag wip --domain rust
No notes in domain 'rust' with tag 'wip'.
```