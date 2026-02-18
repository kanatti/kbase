# Parsing

How `kb` extracts structured data from markdown notes.

---

## What We Need to Parse

From each `.md` file:

| Element | Example |
|---|---|
| Headings | `# Title`, `## Section` |
| Wikilinks | `[[note]]`, `[[domain/note]]`, `[[note\|alias]]`, `[[note#section]]` |
| Tags | `#deep-dive`, `#wip` |
| Tasks | `- [ ] todo`, `- [x] done`, `- [-] cancelled` |

---

## Phase 1: pulldown-cmark + Regex

Current approach. Two tools covering different concerns:

**`pulldown-cmark`** for standard markdown structure:
- Headings (level + text)
- Standard links and images
- List items (needed to find task lines)

**Regex** for Obsidian-specific syntax that pulldown-cmark doesn't understand:

```rust
// Wikilinks: [[note]], [[domain/note]], [[note|alias]], [[note#section]]
let wikilink = Regex::new(r"\[\[([^\]|#]+)(?:#([^\]|]+))?(?:\|([^\]]+))?\]\]");

// Tags: #tag (must not be start of line, to avoid confusing with headings)
let tag = Regex::new(r"(?m)(?:^|\s)#([a-zA-Z][a-zA-Z0-9_-]*)");

// Tasks: - [ ], - [x], - [-], - [?]
let task = Regex::new(r"^- \[(.)\] (.+)");
```

### Tradeoffs

- Simple, no extra build steps
- Regex is fragile — edge cases like wikilinks inside code blocks, escaped brackets
- Two tools means two mental models for the same file
- Good enough for a personal vault with consistent formatting

---

## Phase 2: Custom Tree-sitter Grammar

A proper parser that handles both standard markdown and Obsidian-specific syntax
in a single unified grammar.

### Why Tree-sitter

- One parser, one mental model
- Proper CST — no ambiguity, no edge case surprises
- Declarative queries instead of imperative regex matching
- Incremental parsing — only re-parse changed parts of a file
- Reusable — the grammar is a standalone project useful beyond `kb`

### Grammar Sketch

Written in tree-sitter's `grammar.js` format:

```javascript
// tree-sitter-obsidian-md
module.exports = grammar({
  name: 'obsidian_md',
  rules: {
    document: $ => repeat($._block),

    _block: $ => choice(
      $.heading,
      $.task,
      $.paragraph,
    ),

    heading: $ => seq(
      field('marker', /#{1,6}/),
      field('text', $.inline),
    ),

    task: $ => seq(
      '- [',
      field('status', /[ x\-?]/),
      '] ',
      field('content', /.+/),
    ),

    wikilink: $ => seq(
      '[[',
      field('target', /[^\]|#]+/),
      optional(seq('#', field('section', /[^\]|]+/))),
      optional(seq('|', field('alias', /[^\]]+/))),
      ']]',
    ),

    tag: $ => seq(
      '#',
      field('name', /[a-zA-Z][a-zA-Z0-9_-]*/),
    ),
  }
});
```

### Query Example

```scheme
; headings
(heading marker: _ @marker text: (inline) @text)

; wikilinks
(wikilink target: _ @target)

; tasks
(task status: _ @status content: _ @content)

; tags
(tag name: _ @name)
```

### Build Pipeline

```
grammar.js
    ↓ tree-sitter generate
src/parser.c   (generated C)
    ↓ build.rs (cc crate)
compiled into kb binary
```

### Separate Repo

The grammar should live as its own project — `tree-sitter-obsidian-md` — so it
can be used by other tools (editors, language servers, other CLIs).

`kb` would depend on it via Cargo:
```toml
tree-sitter = "0.22"
tree-sitter-obsidian-md = { git = "https://github.com/kanatti/tree-sitter-obsidian-md" }
```

### Tradeoffs

- Significant upfront investment (grammar + build pipeline + queries)
- Cleaner long-term, especially as parsing needs grow
- Grammar is a reusable artifact beyond `kb`
- Requires learning tree-sitter grammar format

---

## Decision

Phase 1 ships with `pulldown-cmark` + regex. Migrate to tree-sitter when:
- Regex edge cases become a real problem
- The grammar is ready as a standalone project
- Incremental parsing becomes worth it (larger vaults, slower builds)
