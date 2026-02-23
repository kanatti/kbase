# Link Index Implementation

**Status**: Planning  
**Date**: 2026-02-22  
**Goal**: Build bidirectional link index from wikilinks for navigation and context assembly

---

## API

### Commands

```bash
# Build the index
kbase index --only links

# Query links (default: depth 1)
kbase links <note>                      # Show both forward and backward links
kbase links <note> --forward            # Show only outgoing links from this note
kbase links <note> --backward           # Show only incoming links to this note (backlinks)
kbase links <note> --depth N            # Traverse N levels deep (default: 1)
kbase links <note> --forward --depth 2  # Multi-level forward traversal
kbase links <note> --json               # JSON output (for agents/scripts)
```

### Output Format

#### Tree Format (Default)

**Default (both directions, depth 1):**
```
Links for lucene/search-flow.md

Forward links (2):
lucene/search-flow.md
├── lucene/codecs.md
└── lucene/bkd-trees.md

Backward links (3):
lucene/search-flow.md
├── lucene/01-home.md
├── datafusion/query-execution.md
└── elasticsearch/esql-analysis.md
```

**Forward only with depth 2 (`--forward --depth 2`):**
```
Forward links from lucene/search-flow.md (depth: 2, 5 total)

lucene/search-flow.md
├── lucene/codecs.md
│   ├── lucene/postings.md
│   └── lucene/index-format.md
└── lucene/bkd-trees.md
    └── lucene/spatial-index.md
```

**Backward only with depth 2 (`--backward --depth 2`):**
```
Backward links to lucene/codecs.md (depth: 2, 4 total)

lucene/codecs.md
├── lucene/search-flow.md
│   ├── lucene/01-home.md
│   └── datafusion/query-execution.md
└── lucene/index-format.md
    └── elasticsearch/esql-analysis.md
```

**Multiple paths to same note (shows all occurrences):**
```
Forward links from lucene/note-a.md (depth: 2, 4 total)

lucene/note-a.md
├── lucene/note-b.md
│   └── lucene/note-d.md
└── lucene/note-c.md
    └── lucene/note-d.md  (appears again via different path)
```

Note: `note-d.md` appears twice (two different paths), but its children are only traversed once (prevents cycles).

**No links:**
```
No links found for lucene/orphan-note.md
```

#### JSON Format (`--json`)

**Consistent structure across all queries:**
```json
{
  "note": "lucene/search-flow.md",
  "depth": 1,
  "forward": {
    "total": 2,
    "links": [...]
  },
  "backward": {
    "total": 3,
    "links": [...]
  }
}
```

**Default (both directions, depth 1):**
```json
{
  "note": "lucene/search-flow.md",
  "depth": 1,
  "forward": {
    "total": 2,
    "links": [
      { "path": "lucene/codecs.md", "depth": 1 },
      { "path": "lucene/bkd-trees.md", "depth": 1 }
    ]
  },
  "backward": {
    "total": 3,
    "links": [
      { "path": "lucene/01-home.md", "depth": 1 },
      { "path": "datafusion/query-execution.md", "depth": 1 },
      { "path": "elasticsearch/esql-analysis.md", "depth": 1 }
    ]
  }
}
```

**Forward only (`--forward --json`):**
```json
{
  "note": "lucene/search-flow.md",
  "depth": 1,
  "forward": {
    "total": 2,
    "links": [
      { "path": "lucene/codecs.md", "depth": 1 },
      { "path": "lucene/bkd-trees.md", "depth": 1 }
    ]
  }
}
```

**Backward only (`--backward --json`):**
```json
{
  "note": "lucene/search-flow.md",
  "depth": 1,
  "backward": {
    "total": 3,
    "links": [
      { "path": "lucene/01-home.md", "depth": 1 },
      { "path": "datafusion/query-execution.md", "depth": 1 },
      { "path": "elasticsearch/esql-analysis.md", "depth": 1 }
    ]
  }
}
```

**Forward with depth 2 (`--forward --depth 2 --json`):**
```json
{
  "note": "lucene/search-flow.md",
  "depth": 2,
  "forward": {
    "total": 5,
    "links": [
      {
        "path": "lucene/codecs.md",
        "depth": 1,
        "children": [
          { "path": "lucene/postings.md", "depth": 2 },
          { "path": "lucene/index-format.md", "depth": 2 }
        ]
      },
      {
        "path": "lucene/bkd-trees.md",
        "depth": 1,
        "children": [
          { "path": "lucene/spatial-index.md", "depth": 2 }
        ]
      }
    ]
  }
}
```

**No links:**
```json
{
  "note": "lucene/orphan-note.md",
  "depth": 1,
  "forward": {
    "total": 0,
    "links": []
  },
  "backward": {
    "total": 0,
    "links": []
  }
}
```

**Notes:**
- Always includes `note` and `depth`
- Only includes `forward` and/or `backward` as requested
- `links` array contains nested `children` for depth > 1
- `total` counts all unique nodes across all depths

### Error Cases

**Note doesn't exist:**
```
Error: note not found: lucene/nonexistent.md
```

**Index not built:**
```
Error: link index not found. Run 'kbase index --only links' first.
```

---

## Storage

### Files

Two separate JSON files in `~/.kbase/<vault-name>/`:

1. **`links-forward.json`** - Maps source notes to their targets (outgoing links)
2. **`links-backward.json`** - Maps target notes to their sources (incoming links/backlinks)

### Format

**`links-forward.json`:**
```json
{
  "lucene/search-flow.md": [
    "lucene/codecs.md",
    "lucene/bkd-trees.md"
  ],
  "lucene/01-home.md": [
    "lucene/search-flow.md",
    "lucene/indexing-basics.md"
  ],
  "datafusion/query-execution.md": [
    "lucene/search-flow.md"
  ]
}
```

**`links-backward.json`:**
```json
{
  "lucene/codecs.md": [
    "lucene/search-flow.md"
  ],
  "lucene/bkd-trees.md": [
    "lucene/search-flow.md"
  ],
  "lucene/search-flow.md": [
    "datafusion/query-execution.md",
    "lucene/01-home.md"
  ]
}
```

### Storage Properties

- **Keys**: Vault-relative note paths (e.g., `domain/note.md` or `note.md`)
- **Values**: Arrays of vault-relative note paths
- **Sorting**: Paths sorted alphabetically within each array
- **Resolved only**: Only successfully resolved links are stored
- **No metadata**: No line numbers, aliases, or sections (keep it simple)
- **Section stripping**: `[[note#section]]` → stored as `note.md`
- **Atomic writes**: Write to temp file, then rename

### What Gets Stored

✅ **Included:**
- Resolved wikilinks: `[[note]]`, `[[domain/note]]`
- Section links (section stripped): `[[note#section]]` → `note.md`
- Alias links: `[[note|alias]]` → `note.md` (alias discarded)

❌ **Excluded:**
- Unresolved links (broken wikilinks)
- Markdown links: `[text](url)`
- Links in code blocks (tree-sitter already filters these)

### Loading Strategy

**Query-time loading:**
- `kbase links <note>` (default): Load both files
- `kbase links <note> --forward`: Load only `links-forward.json`
- `kbase links <note> --backward`: Load only `links-backward.json`

Both files are small (typically < 100KB even for large vaults), so loading both is fast.

---

## Indexing Steps

### High-Level Flow

```
1. Walk vault → collect all note paths
2. Build stem → paths lookup map
3. Parse each note → extract wikilinks
4. Resolve wikilinks → actual paths
5. Build forward map (source → targets)
6. Build backward map (target → sources)
7. Save both maps to JSON
```

### Detailed Steps

#### Step 1: Collect All Note Paths

```rust
let all_notes: HashSet<PathBuf> = vault
    .walk_notes()
    .map(|note| note.path)
    .collect();
```

**Purpose**: Fast O(1) lookup during resolution to check if a path exists.

**Data**: `all_notes: HashSet<PathBuf>`

---

#### Step 2: Build Stem Lookup Map

```rust
let mut stem_to_paths: HashMap<String, Vec<PathBuf>> = HashMap::new();

for path in &all_notes {
    let stem = path.file_stem().unwrap().to_str().unwrap();
    stem_to_paths
        .entry(stem.to_string())
        .or_default()
        .push(path.clone());
}
```

**Purpose**: Resolve bare wikilinks like `[[note]]` to actual paths.

**Example**:
```rust
{
  "search-flow": ["lucene/search-flow.md"],
  "01-home": ["lucene/01-home.md", "datafusion/01-home.md"], // Ambiguous!
  "codecs": ["lucene/codecs.md"]
}
```

**Data**: `stem_to_paths: HashMap<String, Vec<PathBuf>>`

---

#### Step 3: Parse Notes and Extract Wikilinks

```rust
let mut raw_links: HashMap<PathBuf, Vec<Wikilink>> = HashMap::new();

for note_path in &all_notes {
    let content = vault.read_note(&note_path)?;
    let parsed = parser.parse(&content)?;
    raw_links.insert(note_path.clone(), parsed.wikilinks);
}
```

**Purpose**: Extract raw wikilinks from each note using tree-sitter parser.

**Data**: `raw_links: HashMap<PathBuf, Vec<Wikilink>>`

**Wikilink struct** (from parser):
```rust
pub struct Wikilink {
    pub target: String,          // "domain/note" or "note" or "note#section"
    pub alias: Option<String>,   // Display text (ignored for indexing)
    pub section: Option<String>, // Heading anchor (stripped for indexing)
    pub line: usize,
    pub column: usize,
}
```

---

#### Step 4: Resolve Wikilinks and Build Maps

```rust
let mut forward: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();
let mut backward: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();
let mut unresolved_count = 0;

for (source_path, wikilinks) in raw_links {
    for wikilink in wikilinks {
        // Strip section: [[note#section]] → "note"
        let target = wikilink.target.split('#').next().unwrap();
        
        if let Some(resolved_path) = resolve_target(
            target,
            &source_path,
            &all_notes,
            &stem_to_paths
        ) {
            // Add to forward map: source → target
            forward
                .entry(source_path.clone())
                .or_default()
                .push(resolved_path.clone());
            
            // Add to backward map: target → source
            backward
                .entry(resolved_path)
                .or_default()
                .push(source_path.clone());
        } else {
            unresolved_count += 1;
        }
    }
}

// Sort all path lists alphabetically
for targets in forward.values_mut() {
    targets.sort();
    targets.dedup(); // Remove duplicates
}
for sources in backward.values_mut() {
    sources.sort();
    sources.dedup();
}
```

**Purpose**: Turn raw wikilink strings into actual file paths, build both directions.

**Data**:
- `forward: HashMap<PathBuf, Vec<PathBuf>>`
- `backward: HashMap<PathBuf, Vec<PathBuf>>`
- `unresolved_count: usize`

---

#### Step 5: Save to Disk

```rust
let index_dir = vault.index_dir()?;

save_json(&forward, &index_dir.join("links-forward.json"))?;
save_json(&backward, &index_dir.join("links-backward.json"))?;

println!("Built link index:");
println!("  {} notes with outgoing links", forward.len());
println!("  {} notes with incoming links", backward.len());
if unresolved_count > 0 {
    println!("  {} unresolved links (broken)", unresolved_count);
}
```

**Purpose**: Persist both maps atomically.

**Helper**:
```rust
fn save_json<T: Serialize>(data: &T, path: &Path) -> Result<()> {
    let tmp_path = path.with_extension("tmp");
    let json = serde_json::to_string_pretty(data)?;
    std::fs::write(&tmp_path, json)?;
    std::fs::rename(&tmp_path, path)?; // Atomic on most systems
    Ok(())
}
```

---

## Depth Traversal Algorithm

### Overview

Multi-level link traversal explores the link graph to a specified depth, preventing infinite loops while showing all paths to each node.

### Behavior

- **Default depth**: 1 (immediate links only)
- **Max depth**: Configurable via `--depth N`
- **Cycle prevention**: Visited set prevents traversing same node's children multiple times
- **All paths shown**: Node can appear multiple times if reachable via different paths
- **Breadth-first**: Explores level by level

### Example: Multiple Paths

**Graph:**
```
A → B → D
A → C → D
D → E
```

**Depth 2 from A:**
```
A
├── B
│   └── D
│       └── E
└── C
    └── D  (appears again, but children not traversed - already visited)
```

**Why D appears twice**: Two different paths (A→B→D and A→C→D)  
**Why E appears once**: D's children only traversed on first visit

### Algorithm Pseudocode

```rust
fn traverse_links(
    start: &Path,
    direction: Direction,  // Forward or Backward
    max_depth: usize,
    index: &LinkIndex,
) -> Vec<LinkNode> {
    let mut visited = HashSet::new();  // Tracks nodes we've expanded
    let mut queue = VecDeque::new();
    let mut result = Vec::new();
    
    queue.push_back((start.clone(), 0, None));  // (node, depth, parent)
    
    while let Some((current, depth, parent)) = queue.pop_front() {
        if depth > max_depth {
            continue;
        }
        
        // Record this node in result
        let node = LinkNode {
            path: current.clone(),
            depth,
            parent,
            children: Vec::new(),
        };
        result.push(node);
        
        // Only traverse children if we haven't visited this node before
        if !visited.contains(&current) && depth < max_depth {
            visited.insert(current.clone());
            
            // Get links in the specified direction
            let links = match direction {
                Direction::Forward => index.get_forward(&current),
                Direction::Backward => index.get_backward(&current),
            };
            
            if let Some(links) = links {
                for target in links {
                    queue.push_back((target.clone(), depth + 1, Some(current.clone())));
                }
            }
        }
    }
    
    result
}
```

### Edge Cases

**Self-loops:**
```
A → A
```
Result: A appears twice (root + child), but second occurrence doesn't expand (visited).

**Diamond pattern:**
```
A → B → D
A → C → D
```
Result: D appears twice, children expanded once.

**Deep cycles:**
```
A → B → C → A
```
With depth 3: Shows A→B→C→A, but A's children not re-expanded (visited).

---

## Resolution Algorithm

### Function Signature

```rust
fn resolve_target(
    target: &str,
    source_path: &Path,
    all_notes: &HashSet<PathBuf>,
    stem_to_paths: &HashMap<String, Vec<PathBuf>>,
) -> Option<PathBuf>
```

### Algorithm

```rust
fn resolve_target(
    target: &str,
    source_path: &Path,
    all_notes: &HashSet<PathBuf>,
    stem_to_paths: &HashMap<String, Vec<PathBuf>>,
) -> Option<PathBuf> {
    // Case 1: Path-style target (contains '/')
    // Example: [[lucene/codecs]] or [[domain/note]]
    if target.contains('/') {
        let candidate = PathBuf::from(format!("{}.md", target));
        
        // Direct lookup: does this exact path exist?
        if all_notes.contains(&candidate) {
            return Some(candidate);
        }
        
        // Path-style targets MUST be exact - no fallback
        return None;
    }
    
    // Case 2: Bare name target (no '/')
    // Example: [[codecs]], [[search-flow]]
    
    // 2a. Try same-domain relative lookup first
    if let Some(domain) = source_path.parent() {
        let candidate = domain.join(format!("{}.md", target));
        
        if all_notes.contains(&candidate) {
            return Some(candidate);
        }
    }
    
    // 2b. Try stem lookup across all domains
    if let Some(paths) = stem_to_paths.get(target) {
        match paths.len() {
            0 => return None,           // Not found
            1 => return Some(paths[0].clone()), // Unambiguous match
            _ => return None,           // Ambiguous (multiple matches, skip)
        }
    }
    
    // Not found
    None
}
```

### Resolution Examples

**Source**: `lucene/search-flow.md`

| Wikilink | Resolution | Result |
|----------|------------|--------|
| `[[codecs]]` | Same domain → `lucene/codecs.md` exists | ✅ `lucene/codecs.md` |
| `[[lucene/codecs]]` | Path-style → exact match | ✅ `lucene/codecs.md` |
| `[[bkd-trees]]` | Same domain → `lucene/bkd-trees.md` exists | ✅ `lucene/bkd-trees.md` |
| `[[query-execution]]` | Same domain fails → stem lookup → `datafusion/query-execution.md` | ✅ `datafusion/query-execution.md` |
| `[[01-home]]` | Same domain fails → stem lookup → multiple matches | ❌ Ambiguous |
| `[[nonexistent]]` | Same domain fails → stem lookup fails | ❌ Not found |
| `[[codecs#Overview]]` | Strip section → resolve `codecs` | ✅ `lucene/codecs.md` |

### Edge Cases

**Ambiguous stems:**
- `[[01-home]]` when both `lucene/01-home.md` and `datafusion/01-home.md` exist
- **Resolution**: None (skip, report as unresolved)
- **Recommendation**: Use path-style `[[lucene/01-home]]`

**Cross-domain links:**
- `[[datafusion/query-execution]]` from `lucene/search-flow.md`
- **Resolution**: Path-style, direct match
- **Works**: Yes, if path exists

**Root-level notes:**
- `[[glossary]]` when `glossary.md` exists at vault root
- **Resolution**: No domain parent, goes to stem lookup
- **Works**: Yes, if unambiguous

**Section links:**
- `[[note#Introduction]]`
- **Resolution**: Strip `#Introduction`, resolve `note`
- **Stored as**: `note.md` (section discarded)
- **Future**: Could enhance to validate section exists

---

## Data Structures

### In-Memory (during indexing)

```rust
struct LinkIndexBuilder {
    // Input data
    all_notes: HashSet<PathBuf>,                     // All note paths
    stem_to_paths: HashMap<String, Vec<PathBuf>>,    // Stem → paths lookup
    
    // Intermediate data
    raw_links: HashMap<PathBuf, Vec<Wikilink>>,      // Parsed wikilinks per note
    
    // Output data
    forward: HashMap<PathBuf, Vec<PathBuf>>,         // Source → targets
    backward: HashMap<PathBuf, Vec<PathBuf>>,        // Target → sources
    
    // Statistics
    unresolved_count: usize,
}
```

### On-Disk (persisted)

```rust
// links-forward.json
type ForwardLinks = HashMap<String, Vec<String>>;  // source → targets

// links-backward.json
type BackwardLinks = HashMap<String, Vec<String>>; // target → sources
```

JSON keys/values are strings (vault-relative paths), not `PathBuf`.

### Query-Time (loaded)

```rust
struct LinkIndex {
    forward: HashMap<PathBuf, Vec<PathBuf>>,
    backward: HashMap<PathBuf, Vec<PathBuf>>,
}

impl LinkIndex {
    fn load(vault: &Vault) -> Result<Self> {
        let index_dir = vault.index_dir()?;
        
        let forward = load_json(&index_dir.join("links-forward.json"))?;
        let backward = load_json(&index_dir.join("links-backward.json"))?;
        
        Ok(Self { forward, backward })
    }
    
    fn load_forward_only(vault: &Vault) -> Result<HashMap<PathBuf, Vec<PathBuf>>> {
        let index_dir = vault.index_dir()?;
        load_json(&index_dir.join("links-forward.json"))
    }
    
    fn load_backward_only(vault: &Vault) -> Result<HashMap<PathBuf, Vec<PathBuf>>> {
        let index_dir = vault.index_dir()?;
        load_json(&index_dir.join("links-backward.json"))
    }
    
    fn get_forward(&self, note: &Path) -> Option<&[PathBuf]> {
        self.forward.get(note).map(|v| v.as_slice())
    }
    
    fn get_backward(&self, note: &Path) -> Option<&[PathBuf]> {
        self.backward.get(note).map(|v| v.as_slice())
    }
}
```

---

## Performance Considerations

### Indexing

**Time Complexity**:
- Walk notes: O(N) where N = number of notes
- Parse notes: O(N × M) where M = avg note size
- Resolve links: O(L × log N) where L = total links (HashMap lookup)
- Build maps: O(L)
- **Total**: O(N × M + L × log N)

**Space Complexity**:
- `all_notes`: O(N)
- `stem_to_paths`: O(N)
- `raw_links`: O(L)
- `forward` + `backward`: O(L)
- **Total**: O(N + L)

**Typical vault** (600 notes, ~10 links/note):
- N = 600
- L = 6000
- Indexing time: < 1 second
- Memory: < 10 MB

### Querying

**Time Complexity**:
- Load JSON: O(L) where L = total links in index
- Lookup: O(1) HashMap access
- **Total**: O(L) load + O(1) query

**Space Complexity**: O(L) for loaded index

**Optimization**: For `--forward` or `--backward` only, load just one file.

---

## Future Enhancements

### Phase 1 (Current Scope)
- ✅ Build forward/backward link index
- ✅ Query links with `--forward`/`--backward` flags
- ✅ Resolve wikilinks (path-style, bare name, same-domain priority)

### Phase 2 (Future)
- [ ] Track unresolved links per note (for validation)
- [ ] Section-aware resolution: validate `[[note#section]]` heading exists
- [ ] Line numbers and aliases in index (for richer output)
- [ ] Link metadata: `LinkReference { source, target, line, alias, section }`

### Phase 3 (Advanced)
- [ ] `kbase links validate` - find broken links
- [ ] `kbase links suggest <broken>` - fuzzy match suggestions
- [ ] `kbase graph orphans` - notes with no links in/out
- [ ] `kbase graph hubs` - highly connected notes

---

## Testing Strategy

### Unit Tests

**Resolution algorithm:**
```rust
#[test]
fn test_resolve_path_style() {
    // [[lucene/codecs]] → lucene/codecs.md
}

#[test]
fn test_resolve_bare_name_same_domain() {
    // Source: lucene/search-flow.md
    // [[codecs]] → lucene/codecs.md
}

#[test]
fn test_resolve_bare_name_cross_domain() {
    // Source: lucene/search-flow.md
    // [[query-execution]] → datafusion/query-execution.md (if unique)
}

#[test]
fn test_resolve_ambiguous() {
    // [[01-home]] → None (multiple domains have it)
}

#[test]
fn test_resolve_with_section() {
    // [[note#section]] → strip section, resolve note
}
```

**Index building:**
```rust
#[test]
fn test_build_forward_index() {
    // Parse notes, build forward map
}

#[test]
fn test_build_backward_index() {
    // Invert forward map correctly
}

#[test]
fn test_unresolved_links_counted() {
    // Track broken links
}
```

### Integration Tests

**Fixture vault:**
```
tests/fixtures/vault/
  lucene/
    01-home.md         → links to [[search-flow]]
    search-flow.md     → links to [[codecs]], [[datafusion/query-execution]]
    codecs.md
  datafusion/
    01-home.md         → links to [[query-execution]]
    query-execution.md → links to [[lucene/search-flow]]
```

**Tests:**
```rust
#[test]
fn test_index_and_query_forward() {
    // kbase index --only links
    // kbase links lucene/search-flow.md --forward
    // Assert: shows codecs.md and datafusion/query-execution.md
}

#[test]
fn test_index_and_query_backward() {
    // kbase links lucene/search-flow.md --backward
    // Assert: shows lucene/01-home.md and datafusion/query-execution.md
}

#[test]
fn test_index_and_query_both() {
    // kbase links lucene/search-flow.md
    // Assert: shows both forward and backward
}
```

---

## Implementation Checklist

### Core Functionality
- [ ] Add `links-forward.json` and `links-backward.json` storage
- [ ] Implement `resolve_target()` algorithm
- [ ] Build link index during `kbase index --only links`
- [ ] Add `kbase links <note>` command
- [ ] Add `--forward` and `--backward` flags
- [ ] Load index files on demand (optimize for flag usage)

### Error Handling
- [ ] Handle missing index files (suggest running `kbase index`)
- [ ] Handle nonexistent note paths
- [ ] Report unresolved link count during indexing

### Output Formatting
- [ ] Format forward links with `→` arrows
- [ ] Format backward links with `←` arrows
- [ ] Show counts: "Forward links (2):", "Backward links (3):"
- [ ] Handle empty results: "No links found for X"

### Testing
- [ ] Unit tests for resolution algorithm
- [ ] Integration tests with fixture vault
- [ ] Test all flag combinations

### Documentation
- [ ] Update `docs/` with link index documentation
- [ ] Add examples to CLI help text
- [ ] Update README with link navigation features

---

## Related Documents

- `plan/agentic.md` - Link index is Phase 1 foundation feature
- `plan/wikilinks-agentic.md` - Broader wikilink feature brainstorming
- `src/parser/tree_sitter.rs` - Wikilink extraction implementation
- `docs/tree-sitter.md` - Tree-sitter integration details
