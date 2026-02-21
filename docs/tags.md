# Tags

Inline `#tags` for categorizing and discovering notes across your vault.

## Usage

```bash
kb tags                                      # list all tags
kb tags --sort count                         # sort by usage frequency
kb notes --tag deep-dive                     # filter notes by tag
kb notes --tag wip --domain lucene           # combine tag + domain filter
kb index                                     # build tag index
kb index --only tags                         # build only tag index
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
~/.kb/<vault-name>/tags.json
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

### `kb tags`

List all tags with note counts.

```bash
kb tags                          # alphabetical (default)
kb tags --sort name              # alphabetical (explicit)
kb tags --sort count             # by usage frequency
```

Output:

```
Tag          Notes
deep-dive    2
performance  1
wip          1
```

### `kb notes --tag`

Filter notes by tag. Requires tag index to be built first.

```bash
kb notes --tag deep-dive                      # all notes with this tag
kb notes --tag deep-dive --domain lucene      # tag + domain filter
kb notes --tag wip --files                    # show paths only
```

Output matches `kb notes` format (table or paths).

### `kb index`

Build tag index by scanning vault content.

```bash
kb index                         # build all indexes (tags, links, search)
kb index --only tags             # build only tag index
```

## Error Messages

**Missing index:**
```bash
$ kb notes --tag deep-dive
No tag index found. Run `kb index` to build it first.
```

**Tag not found:**
```bash
$ kb notes --tag nonexistent
No notes with tag 'nonexistent'.
```

**Tag + domain no results:**
```bash
$ kb notes --tag wip --domain rust
No notes in domain 'rust' with tag 'wip'.
```