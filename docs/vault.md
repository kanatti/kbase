# Vault

`Vault` is the core abstraction in `kb`. It represents a knowledge base directory containing markdown files organized into domains.

---

## Structure

```rust
pub struct Vault {
    pub root: PathBuf,    // vault root directory
    pub name: String,     // vault name from config
}
```

Simple and lightweight — no cached indexes or complex in-memory structures. The vault provides methods to scan and read files on demand.

## Domain

Top-level directories become domains:

```rust
pub struct Domain {
    pub name: String,        // "elasticsearch"
    pub path: PathBuf,       // /vault/elasticsearch  
    pub note_count: usize,   // number of .md files inside
}
```

**Domain Rules:**
- Only directories at vault root
- Names cannot start with `_` or `.` (excludes `__templates`, `_logs`, etc.)
- Empty directories are included (with count 0)

## Note

Individual markdown files:

```rust
pub struct Note {
    pub path: PathBuf,    // relative to vault: "elasticsearch/esql-analysis.md"
    pub filename: String, // "esql-analysis.md"
    pub title: String,    // first # heading, or filename stem if none
}
```

Notes are created on-demand by scanning directories. The title is extracted by reading the first few lines to find a `# Heading`.

## Operations

### Domain Listing

```rust
impl Vault {
    pub fn domains(&self) -> Result<Vec<Domain>> {
        // Scan vault root for directories
        // Count .md files in each
        // Return sorted by name
    }
}
```

### Note Listing

```rust  
impl Vault {
    pub fn all_notes(&self) -> Result<Vec<Note>> {
        // Walk all domains, collect all .md files
    }
    
    pub fn notes_in_domain(&self, domain: &str) -> Result<Vec<Note>> {
        // Scan single domain directory
    }
}
```

### Note Reading

```rust
impl Vault {
    pub fn read_note(&self, path: &str) -> Result<String> {
        // Read file content by vault-relative path
    }
}
```

## Index Storage

Indexes are stored separately from the vault structure:

```
~/.kb/
├── config.toml
└── indexes/
    └── <vault-name>/
        ├── tags.json      # tag → [note paths]
        ├── links.json     # link graph (future)  
        └── search.tantivy/ # full-text index (future)
```

Each vault gets its own index directory. Indexes are built by `kb index` and consumed by various commands (`kb tags`, `kb notes --tag`, etc.).

## File System Rules

**Included:**
- All `.md` files in domain directories
- Domain directories (non-hidden, don't start with `_`)

**Excluded:**  
- `__templates/`, `__attachments/` — utility directories
- `_logs/`, `_planning/` — private directories
- Hidden files/dirs (starting with `.`)
- Non-markdown files

**Root files:**
Files at vault root (like `01-home.md`) are included but don't belong to any domain.

## Performance

No in-memory caching — everything scans the filesystem on each command. This keeps the implementation simple and always reflects the current state of files.

Typical performance on 658 notes:
- Domain listing: ~5ms
- Note listing: ~20ms  
- Note reading: ~1ms

Fast enough for interactive use while keeping the codebase minimal.
