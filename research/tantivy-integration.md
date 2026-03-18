# Tantivy API Reference

Quick reference for using Tantivy in kbase search implementation.

**Version:** 0.22  
**Use case:** CLI tool, 600-1000 markdown notes, full rebuild on each index

---

## Quick Reference

### Core Types

```rust
use tantivy::{Index, IndexWriter, TantivyDocument, doc};
use tantivy::schema::*;
use tantivy::query::QueryParser;
use tantivy::collector::TopDocs;
```

### Minimal Example

```rust
// 1. Schema
let mut schema_builder = Schema::builder();
schema_builder.add_text_field("path", STORED);
schema_builder.add_text_field("domain", STRING | STORED);
schema_builder.add_text_field("title", TEXT | STORED);
schema_builder.add_text_field("content", TEXT);
let schema = schema_builder.build();

// 2. Create index
let index = Index::create_in_dir(path, schema.clone())?;
let mut writer = index.writer(50_000_000)?;

// 3. Index documents
let path_field = schema.get_field("path").unwrap();
let domain_field = schema.get_field("domain").unwrap();
let title_field = schema.get_field("title").unwrap();
let content_field = schema.get_field("content").unwrap();

writer.add_document(doc!(
    path_field => "lucene/search.md",
    domain_field => "lucene",
    title_field => "Search Architecture",
    content_field => "Full markdown content...",
))?;

writer.commit()?;

// 4. Search
let index = Index::open_in_dir(path)?;
let reader = index.reader()?;
let searcher = reader.searcher();

let mut parser = QueryParser::for_index(&index, vec![title_field, content_field]);
parser.set_field_boost(title_field, 2.0);

let query = parser.parse_query("search query")?;
let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

for (_score, doc_address) in top_docs {
    let doc = searcher.doc(doc_address)?;
    let title = doc.get_first(title_field)
        .and_then(|v| v.as_str())
        .unwrap_or("");
    println!("{}", title);
}
```

---

## Schema

### Field Types & Flags

| Flag | Indexed | Stored | Tokenized | Use Case |
|------|---------|--------|-----------|----------|
| `STORED` | ❌ | ✅ | N/A | Metadata only (retrieve but don't search) |
| `STRING` | ✅ | ❌ | ❌ | Exact match filters (not tokenized) |
| `STRING \| STORED` | ✅ | ✅ | ❌ | Filterable + retrievable (domain) |
| `TEXT` | ✅ | ❌ | ✅ | Full-text search only |
| `TEXT \| STORED` | ✅ | ✅ | ✅ | Full-text + retrievable (title) |

### Schema Definition

```rust
use tantivy::schema::*;

let mut builder = Schema::builder();

// Path: retrieve only (not searchable)
builder.add_text_field("path", STORED);

// Domain: exact match filter + retrieve
builder.add_text_field("domain", STRING | STORED);

// Title: full-text search + retrieve
builder.add_text_field("title", TEXT | STORED);

// Content: full-text search only (not stored)
builder.add_text_field("content", TEXT);

let schema = builder.build();
```

### Get Field Handles

```rust
let path = schema.get_field("path").unwrap();
let domain = schema.get_field("domain").unwrap();
let title = schema.get_field("title").unwrap();
let content = schema.get_field("content").unwrap();
```

**Note:** Field names are case-sensitive.

---

## Index Operations

### Create Index

```rust
use tantivy::Index;
use std::path::Path;

// Create new index (fails if exists)
let index = Index::create_in_dir(path, schema)?;
```

### Open Existing Index

```rust
// Read-only (no lock acquired)
let index = Index::open_in_dir(path)?;
```

### Check If Exists

```rust
fn index_exists(path: &Path) -> bool {
    path.join("meta.json").exists()
}
```

### Rebuild Pattern

```rust
use std::fs;

// Delete old index
if path.exists() {
    fs::remove_dir_all(&path)?;
}
fs::create_dir_all(&path)?;

// Create fresh index
let index = Index::create_in_dir(&path, schema)?;
```

---

## Indexing

### Create Writer

```rust
// Memory budget in bytes (50MB recommended for CLI)
let mut writer = index.writer(50_000_000)?;
```

**Memory allocation:**
- Single-threaded: full budget available
- Multi-threaded: budget divided by thread count
- For 1000 notes: 50MB is sufficient

### Add Documents

```rust
use tantivy::doc;

// Using doc! macro
writer.add_document(doc!(
    path => "note.md",
    domain => "lucene",
    title => "Title",
    content => "Content",
))?;
```

### Commit

```rust
// Atomic commit (blocking)
writer.commit()?;
```

**Important:**
- Blocks until all data flushed to disk
- All-or-nothing (rollback on failure)
- Uncommitted documents are lost on drop/crash

### Error Handling

```rust
for note in notes {
    match index_note(&mut writer, note) {
        Ok(_) => count += 1,
        Err(e) => eprintln!("Failed to index {}: {}", note.path, e),
    }
}

writer.commit()?; // Commits all successful documents
```

---

## Querying

### Create Reader & Searcher

```rust
// Create reader (reusable across queries)
let reader = index.reader()?;

// Get searcher (cheap, create per query)
let searcher = reader.searcher();
```

### Parse Queries

```rust
use tantivy::query::QueryParser;

// Search title and content fields
let mut parser = QueryParser::for_index(
    &index,
    vec![title_field, content_field]
);

// Boost title 2× (matches in title score higher)
parser.set_field_boost(title_field, 2.0);

// Parse user query
let query = parser.parse_query("search terms")?;
```

### Query Syntax

| Pattern | Example | Meaning |
|---------|---------|---------|
| Terms | `rust tantivy` | OR (matches either) |
| Boolean | `rust AND traits` | AND |
| Negation | `search -elasticsearch` | NOT |
| Phrases | `"exact phrase"` | Exact match |
| Field | `title:architecture` | Search specific field |
| Wildcard | `tanti*` | Prefix match |

### Execute Search

```rust
use tantivy::collector::TopDocs;

// Get top N results, ordered by score
let top_docs = searcher.search(
    &query,
    &TopDocs::with_limit(10)
)?;

// Results: Vec<(f32, DocAddress)>
for (score, doc_address) in top_docs {
    let doc = searcher.doc(doc_address)?;
    // Extract fields...
}
```

### Extract Stored Fields

```rust
// Get first value (for single-valued fields)
let title = doc.get_first(title_field)
    .and_then(|v| v.as_str())
    .unwrap_or("");

// Get all values (for multi-valued fields)
let values: Vec<&str> = doc.get_all(title_field)
    .filter_map(|v| v.as_str())
    .collect();
```

---

## Filtering

### Exact Match Filter (Domain)

```rust
use tantivy::query::{BooleanQuery, TermQuery, Query, Occur};
use tantivy::schema::IndexRecordOption;
use tantivy::Term;

// Create domain filter
let domain_term = Term::from_field_text(domain_field, "lucene");
let domain_filter = TermQuery::new(domain_term, IndexRecordOption::Basic);

// Combine with text query
let combined = BooleanQuery::new(vec![
    (Occur::Must, text_query),               // Main search
    (Occur::Must, Box::new(domain_filter)),  // Filter
]);

let results = searcher.search(&combined, &TopDocs::with_limit(10))?;
```

### Multiple Domain Filter (OR)

```rust
let domains = vec!["lucene", "search", "indexing"];

let domain_queries: Vec<_> = domains
    .iter()
    .map(|&d| {
        let term = Term::from_field_text(domain_field, d);
        let query = TermQuery::new(term, IndexRecordOption::Basic);
        (Occur::Should, Box::new(query) as Box<dyn Query>)
    })
    .collect();

let multi_domain_filter = BooleanQuery::new(domain_queries);
```

---

## Common Patterns

### CLI Tool Pattern (Open → Search → Close)

```rust
fn search_cli(index_path: &Path, query_str: &str) -> Result<Vec<String>> {
    // Open index (fast: <10ms)
    let index = Index::open_in_dir(index_path)?;
    let reader = index.reader()?;
    let searcher = reader.searcher();
    
    // Parse and execute query
    let schema = index.schema();
    let title = schema.get_field("title").unwrap();
    let content = schema.get_field("content").unwrap();
    
    let mut parser = QueryParser::for_index(&index, vec![title, content]);
    parser.set_field_boost(title, 2.0);
    
    let query = parser.parse_query(query_str)?;
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
    
    // Extract results
    let results = top_docs
        .into_iter()
        .map(|(_score, addr)| {
            let doc = searcher.doc(addr)?;
            doc.get_first(title)
                .and_then(|v| v.as_str())
                .map(String::from)
                .ok_or_else(|| anyhow::anyhow!("Missing title"))
        })
        .collect::<Result<Vec<_>>>()?;
    
    Ok(results)
    // Index/reader/searcher automatically cleaned up
}
```

**Why this works for CLI:**
- Index open is <10ms (mmap, no data loaded)
- No need to keep index alive between invocations
- Simple, no resource management needed

### Full Rebuild Pattern

```rust
fn rebuild_index(vault_path: &Path, notes: Vec<Note>) -> Result<()> {
    let index_path = vault_path.join(".kbase/search.tantivy");
    
    // Remove old
    if index_path.exists() {
        std::fs::remove_dir_all(&index_path)?;
    }
    std::fs::create_dir_all(&index_path)?;
    
    // Create new
    let schema = create_schema();
    let index = Index::create_in_dir(&index_path, schema.clone())?;
    let mut writer = index.writer(50_000_000)?;
    
    // Get fields
    let path = schema.get_field("path").unwrap();
    let domain = schema.get_field("domain").unwrap();
    let title = schema.get_field("title").unwrap();
    let content = schema.get_field("content").unwrap();
    
    // Index all
    for note in notes {
        writer.add_document(doc!(
            path => note.path,
            domain => note.domain,
            title => note.title,
            content => note.content,
        ))?;
    }
    
    writer.commit()?;
    Ok(())
}
```

**For 600-1000 notes:**
- Full rebuild: <1 second
- Simpler than incremental updates
- No delete tracking needed
- Always consistent with filesystem

---

## Error Handling

### Common Errors

```rust
use tantivy::TantivyError;

match result {
    Err(TantivyError::IndexAlreadyExists) => {
        // Delete directory first
    }
    Err(TantivyError::IoError(e)) if e.kind() == ErrorKind::NotFound => {
        // Index doesn't exist
    }
    Err(TantivyError::LockFailure(..)) => {
        // Another writer is active
    }
    Err(TantivyError::DataCorruption(..)) => {
        // Index corrupted, rebuild needed
    }
    Err(TantivyError::InvalidArgument(msg)) => {
        // Invalid query syntax
    }
    Ok(value) => { /* success */ }
}
```

### Missing Index

```rust
fn search_safe(path: &Path, query: &str) -> Result<Vec<String>> {
    if !path.join("meta.json").exists() {
        bail!("Search index not found. Run 'kbase index' first.");
    }
    
    let index = Index::open_in_dir(path)
        .context("Failed to open search index")?;
    
    // ... rest of search
}
```

### Query Parse Errors

```rust
fn parse_query_safe(parser: &QueryParser, query_str: &str) -> Result<Box<dyn Query>> {
    match parser.parse_query(query_str) {
        Ok(q) => Ok(q),
        Err(_) => {
            // Escape special chars and retry
            let escaped = query_str
                .replace(':', r"\:")
                .replace('(', r"\(")
                .replace(')', r"\)")
                .replace('[', r"\[")
                .replace(']', r"\]");
            
            parser.parse_query(&escaped)
                .context("Invalid query syntax")
        }
    }
}
```

---

## Performance

### Typical Timings (1000 notes × 3KB)

| Operation | Time | Notes |
|-----------|------|-------|
| Index build | 0.5-2s | Depends on I/O speed |
| Index open | <10ms | Mmap, no data loaded |
| Query parse | <1ms | Simple queries |
| Search | 1-10ms | BM25 scoring |
| Commit | 0.5-1s | Flush to disk |

### Memory Usage

| Component | Memory |
|-----------|--------|
| Index open | ~1-5 MB |
| Reader | ~1-2 MB |
| Searcher | ~1-5 MB |
| Query execution | ~5-20 MB |
| **Total per query** | **~10-30 MB** |

### Index Size (1000 notes × 3KB)

| Component | Size |
|-----------|------|
| Raw markdown | ~3 MB |
| Stored fields (compressed) | ~5-10 MB |
| Posting lists | ~3-8 MB |
| Positions | ~5-10 MB |
| **Total index** | **~10-30 MB** |

### Optimization for CLI

```rust
// Good defaults for CLI tools:

// 1. Writer memory: 50MB (sufficient for 1000 notes)
let writer = index.writer(50_000_000)?;

// 2. Single-threaded (simpler, predictable)
let writer = index.writer_with_num_threads(1, 50_000_000)?;

// 3. Manual reload policy (no background threads)
let reader = index.reader()?; // Uses Manual by default

// 4. Don't keep index open between CLI invocations
// Open → search → close per command
```

---

## Gotchas

| Issue | Solution |
|-------|----------|
| **Only one writer allowed** | Drop existing writer before creating new one |
| **Schema changes require rebuild** | Delete index directory, recreate with new schema |
| **Field names are case-sensitive** | Use consistent casing: `"title"` not `"Title"` |
| **QueryParser needs default fields** | Specify at least one field: `vec![title, content]` |
| **Readers don't see uncommitted docs** | Call `writer.commit()` then `reader.reload()` |
| **Commit is blocking** | Can take seconds for large indexes |
| **Mmap needs file handles** | Ensure `ulimit -n` is sufficient (not usually an issue) |
| **STORED vs Indexed** | STORED = retrieve, Indexed = search (can have both) |
| **Memory budget per thread** | 4 threads × 50MB = 200MB total |
| **Path must be absolute** | Use `std::env::current_dir()?.join(path)` |

---

## Index Directory Structure

```
~/.kbase/<vault>/search.tantivy/
├── meta.json                    # Index metadata, schema, segment list
├── .managed.json               # Lock tracking
├── .tantivy-writer.lock        # Writer lock (during indexing only)
└── <uuid>.<ext>                # Segment files:
    ├── .store                   #   Compressed stored fields
    ├── .term                    #   Term dictionary (FST)
    ├── .idx                     #   Posting lists
    ├── .pos                     #   Term positions
    ├── .fast                    #   Fast fields (columnar)
    └── .fieldnorm               #   Field length normalization
```

**Key files:**
- `meta.json`: Lists segments, schema, commit opstamp
- `.tantivy-writer.lock`: Prevents multiple writers (auto-deleted on commit)
- Segment files: Immutable (never modified after creation)

---

## Architecture Notes

### Segments

- **Immutable**: Once written, never modified
- **Independent**: Each segment is self-contained
- **Merged**: Tantivy periodically merges segments (automatic)
- **Search**: Queries search all segments, merge results

### BM25 Scoring

- Default relevance algorithm (same as Lucene/Elasticsearch)
- Factors: term frequency, document length, field boost
- Higher score = more relevant
- No configuration needed (automatic)

### Mmap Usage

- Index files are memory-mapped (not loaded into RAM)
- OS manages memory, pages in data as needed
- Very efficient for random access
- Minimal startup time

### Concurrency

| Operation | Concurrency |
|-----------|-------------|
| Multiple readers | ✅ Fully supported, no coordination |
| Multiple writers | ❌ Only one writer (enforced by lock file) |
| Read while writing | ✅ Readers see snapshot before commit |

---

## Quick Checklist

**Building an index:**
- [ ] Create schema with field types
- [ ] Delete old index directory (for rebuild)
- [ ] Create index with `Index::create_in_dir()`
- [ ] Get writer with memory budget (50MB)
- [ ] Add documents with `doc!()` macro
- [ ] Commit atomically

**Searching:**
- [ ] Open index with `Index::open_in_dir()`
- [ ] Create reader
- [ ] Get searcher
- [ ] Parse query with QueryParser
- [ ] Set field boost if needed
- [ ] Execute search with TopDocs
- [ ] Extract stored fields from results

**Error handling:**
- [ ] Check index exists before opening
- [ ] Handle parse errors (escape special chars)
- [ ] Use `anyhow::Context` for error messages
- [ ] Provide helpful messages ("run kbase index first")

---

## Resources

- **Official Docs**: https://docs.rs/tantivy/
- **Examples**: https://github.com/quickwit-oss/tantivy/tree/main/examples
- **Discord**: https://discord.gg/MT27AG5EVE

---

**Document type:** Reference  
**Last updated:** 2026-03-17  
**Tantivy version:** 0.22
