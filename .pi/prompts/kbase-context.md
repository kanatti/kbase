# Knowledge Base Context

Your knowledge base is available via `kbase` commands (using the refer-kbase skill).

## Primary Domains for kbase Development

These domains contain relevant knowledge for working on this project:

- **lucene** - Full-text search architecture, BM25 ranking, inverted indexes, BKD trees
- **tantivy** - Tantivy Rust library implementation, API patterns, indexing
- **tree-sitter** - Parser implementation, query patterns, grammar design
- **rust** - Rust language idioms, ownership, error handling, CLI patterns
- **datafusion** - Query execution, graph traversal, optimization techniques
- **kbase** - This project's own design docs, planning, architecture decisions

## Discovery

Many other domains exist in the vault. To discover them:

```bash
kbase domains                    # List all available domains
kbase notes --domain <domain>    # List notes in a domain
kbase tags                       # List all tags (requires kbase index)
```

## Usage

When topics arise that might have related knowledge:
- Check relevant domains: `kbase notes --domain <domain>`
- Filter by tags: `kbase notes --tag <tag>`
- Read specific notes: `kbase read <domain>/<note>`
