---
name: tantivy-ref
description: Reference for using Tantivy — a full-text search engine library in Rust. Use when implementing search indexing or querying with Tantivy, understanding its API, or navigating the Tantivy source code.
---

# Tantivy Reference

Full-text search engine library in Rust. Architecture strongly inspired by Lucene.
BM25 scoring, persistent inverted index, phrase search, fuzzy matching.

- **Source:** `~/Code/tantivy/`
- **Architecture doc:** `~/Code/tantivy/ARCHITECTURE.md`
- **Examples:** `~/Code/tantivy/examples/`

## Core Concepts

| Tantivy | Lucene equivalent |
|---------|------------------|
| `Index` | IndexWriter + IndexReader |
| `Segment` | Segment |
| `Schema` | Mapping / Schema |
| `IndexWriter` | IndexWriter |
| `Searcher` | IndexSearcher |
| `TopDocs` collector | TopScoreDocCollector |
| Fast fields | DocValues |
| `TEXT` | analyzed text field |
| `STORED` | stored field |

## Minimal API Pattern

### 1. Define schema

```rust
use tantivy::schema::*;

let mut schema_builder = Schema::builder();
let path  = schema_builder.add_text_field("path",  STORED);         // retrieve but don't search
let title = schema_builder.add_text_field("title", TEXT | STORED);  // search + retrieve, boosted
let body  = schema_builder.add_text_field("body",  TEXT);           // search only, no retrieval
let domain = schema_builder.add_text_field("domain", STRING | STORED); // exact match (no tokenize)
let schema = schema_builder.build();
```

Field options:
- `TEXT` — tokenized + indexed (full-text searchable)
- `STRING` — indexed as single token (exact match, good for domain/path)
- `STORED` — saved in doc store (retrievable after search)
- `FAST` — column-oriented storage for sorting/aggregation (doc values)

### 2. Create or open index

```rust
use tantivy::{Index, IndexWriter};
use std::path::Path;

// Create new
let index = Index::create_in_dir(Path::new("~/.kb/index"), schema.clone())?;

// Open existing
let index = Index::open_in_dir(Path::new("~/.kb/index"))?;

// In-memory (for tests)
let index = Index::create_in_ram(schema.clone());
```

### 3. Index documents

```rust
let mut writer: IndexWriter = index.writer(50_000_000)?;  // 50MB buffer

writer.add_document(doc!(
    path  => "lucene/search-flow.md",
    domain => "lucene",
    title => "Search Flow Deep Dive",
    body  => "How TermQuery flows through IndexSearcher...",
))?;

writer.commit()?;  // flush to disk, makes documents searchable
```

### 4. Search

```rust
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::ReloadPolicy;

let reader = index
    .reader_builder()
    .reload_policy(ReloadPolicy::OnCommitWithDelay)
    .try_into()?;

let searcher = reader.searcher();

// Parse query across title + body, title boosted
let query_parser = QueryParser::for_index(&index, vec![title, body]);
let query = query_parser.parse_query("BKD tree")?;

let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

for (_score, doc_address) in top_docs {
    let doc: TantivyDocument = searcher.doc(doc_address)?;
    let path_val = doc.get_first(path).unwrap().as_str().unwrap();
    println!("{}", path_val);
}
```

### 5. Delete documents

```rust
// Delete by term (use path as the unique key)
let path_term = Term::from_field_text(path, "lucene/search-flow.md");
writer.delete_term(path_term);
writer.commit()?;
```

## Planned kb Schema

For `kb notes --term` (Phase 2). See `docs/search.md` for full design.

```rust
let mut schema_builder = Schema::builder();
let path  = schema_builder.add_text_field("path",  STRING | STORED);
let domain = schema_builder.add_text_field("domain", STRING | STORED);
let title = schema_builder.add_text_field("title", TEXT | STORED);   // boosted in queries
let body  = schema_builder.add_text_field("body",  TEXT);            // not stored, saves space
```

Query: `title:"search flow"^2 OR body:"search flow"` — title boost surfaces
notes where term is in the heading above notes that only mention it in passing.

Index location: `~/.kb/index/`

## Key Source Files

```
~/Code/tantivy/src/
  schema/        — Schema, field types, TEXT/STORED/FAST options
  index/         — Index, segment management, meta.json
  indexer/       — IndexWriter, document ingestion, segment writing
  query/         — QueryParser, BooleanQuery, TermQuery, PhraseQuery
  collector/     — TopDocs, Count, DocSetCollector
  tokenizer/     — default tokenizer, token filters, custom tokenizers
  store/         — DocStore (row-oriented, LZ4 compressed)
  fastfield/     — Fast fields (column-oriented, for sorting/facets)
  postings/      — Posting lists, term frequencies, positions
  termdict/      — Term dictionary (FST-based)
```

## Useful Examples

```bash
# Read these in ~/Code/tantivy/examples/
basic_search.rs           — schema → index → search → retrieve (start here)
custom_tokenizer.rs       — custom tokenizer chain
fuzzy_search.rs           — fuzzy term queries
snippet.rs                — highlight matching terms in results
```

## Cargo Dependency

```toml
tantivy = "0.22"
```

## Tips

- `IndexWriter` must be a singleton — only one writer at a time per index
- `Searcher` is cheap to acquire — create one per query, not per process
- Don't `STORE` the body field if you only need it for search (saves significant space)
- Use `STRING` (not `TEXT`) for path and domain — you want exact match, not tokenization
- `writer.commit()` is blocking — call after batch, not per document
- For incremental updates: delete old doc by path term, add new doc, commit
