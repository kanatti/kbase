# kbase Project

Personal knowledge base CLI in Rust. Navigates markdown vaults with wikilinks, tags, and domains (Obsidian format).

## Project Structure

```
kbase/
├── .pi/                  # Pi configuration (meta, project-specific)
├── crates/               # Workspace crates
│   └── tree-sitter-md-obsidian/  # Vendored tree-sitter parser with wikilink/tag support
├── docs/                 # External documentation (user-facing)
├── plan/                 # Planning docs, scratch notes, design discussions
├── scripts/              # Build/install scripts (can be used for automation)
├── src/                  # Source code
└── tests/                # Tests with fixtures in tests/fixtures/vault/
```

## Knowledge Vault Access

The user has a personal knowledge vault. Use the `kbase` CLI to access it (default vault is already configured).

**Domains most relevant to kbase development:**
- **lucene** - Full-text search architecture, BM25 ranking, inverted indexes, BKD trees
- **tantivy** - Tantivy Rust library implementation, API patterns, indexing
- **tree-sitter** - Parser implementation, query patterns, grammar design
- **rust** - Rust language idioms, ownership, error handling, CLI patterns
- **kbase** - This project's own design docs, planning, architecture decisions

The vault contains many other domains. Use `kbase domains` to discover them all.

## Using kbase for kbase Development

Reference the knowledge vault when relevant to development:

```bash
kbase domains                    # List all available domains
kbase notes --domain <domain>    # List notes in a domain
kbase notes --tag <tag>          # Filter by tag (requires kbase index)
kbase tags                       # List all tags
kbase read <domain>/<note>       # Read a specific note
kbase read <note> --outline      # Show heading structure
```

**Examples:**

```bash
# Working on Tantivy integration? Check tantivy notes
kbase notes --domain tantivy

# Need tree-sitter parser patterns?
kbase notes --domain tree-sitter

# Review kbase's own design decisions
kbase notes --domain kbase
kbase read kbase/agentic-features.md
```

Proactively check domains when topics arise (search, parsing, Rust patterns, etc.).
