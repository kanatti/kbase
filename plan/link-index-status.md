# Link Index - Implementation Status

**Date**: 2026-02-22  
**Status**: Core features complete, depth traversal pending

---

## ✅ Completed

### 1. Resolution Logic (`src/links/resolve.rs`)
- **Path-style links**: `[[domain/note]]` → absolute or relative to source
- **Bare names**: `[[note]]` → same domain first, then root level fallback
- **No cross-domain magic**: other domains require explicit paths
- 10 comprehensive tests covering all edge cases

### 2. Index Building (`src/links/index.rs`)
- Single-pass bidirectional index construction
- Forward map: source → targets (outgoing links)
- Backward map: target → sources (backlinks)
- Filters out images (`.png`, `.jpg`, etc.)
- Atomic JSON saves: `links-forward.json`, `links-backward.json`
- Returns unresolved link count

### 3. CLI Integration
- `kbase index --only links` - build link index
- `kbase index` - build both tags and links
- `kbase links <note>` - query links (both directions)
- `kbase links <note> --forward` - outgoing links only
- `kbase links <note> --backward` - backlinks only
- `kbase links <note> --json` - JSON output

### 4. Vault Fix
- `vault.all_notes()` now includes root-level files

---

## 🚧 Not Implemented: Depth Traversal

**Current limitation**: Only shows **depth 1** (immediate/direct links).

**What's needed**: Multi-level graph traversal with cycle prevention.

---

## Next Session: Implementing `--depth N`

### Feature Requirements

**CLI:**
```bash
kbase links <note> --depth 2             # Traverse 2 levels deep
kbase links <note> --forward --depth 3   # 3 levels forward only
kbase links <note> --depth 2 --json      # Nested JSON structure
```

**Default**: `--depth 1` (current behavior)

### Output Changes

#### Text Format (Tree View)

**Current (depth 1):**
```
Links for lucene/search-flow.md

Forward links (2):
  lucene/codecs.md
  lucene/indexing/inverted-index.md
```

**Needed (depth 2):**
```
Forward links from lucene/search-flow.md (depth: 2, 5 total)

lucene/search-flow.md
├── lucene/codecs.md
│   ├── lucene/postings.md
│   └── lucene/index-format.md
└── lucene/indexing/inverted-index.md
    └── lucene/segments.md
```

**Tree drawing characters:**
- `├──` - branch (has siblings after)
- `└──` - last branch (no siblings after)
- `│   ` - continuation line (parent has siblings)
- `    ` - no continuation (parent is last)

#### JSON Format (Nested)

**Current (depth 1):**
```json
{
  "note": "lucene/search-flow.md",
  "depth": 1,
  "forward": {
    "total": 2,
    "links": [
      { "path": "lucene/codecs.md", "depth": 1 },
      { "path": "lucene/indexing/inverted-index.md", "depth": 1 }
    ]
  }
}
```

**Needed (depth 2):**
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
        "path": "lucene/indexing/inverted-index.md",
        "depth": 1,
        "children": [
          { "path": "lucene/segments.md", "depth": 2 }
        ]
      }
    ]
  }
}
```

### Traversal Algorithm

**Key concepts:**
- **Nodes can appear multiple times** (different paths to same node)
- **Children only traversed once** (cycle prevention via visited set)
- **Breadth-first traversal** (level by level)

**Example with cycles:**
```
A → B → C
A → C
C → A
```

**Depth 2 from A:**
```
A
├── B
│   └── C
│       └── A  (appears again, but children not traversed - already visited)
└── C          (appears again, children not traversed - already visited)
```

**Pseudocode:**
```rust
struct LinkNode {
    path: PathBuf,
    depth: usize,
    children: Vec<LinkNode>,  // Only populated if first visit
}

fn traverse_links(
    start: &Path,
    max_depth: usize,
    index: &LinkIndex,
    direction: Direction,  // Forward or Backward
) -> Vec<LinkNode> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut result_map: HashMap<PathBuf, LinkNode> = HashMap::new();
    
    queue.push_back((start.clone(), 0, None)); // (node, depth, parent)
    
    while let Some((current, depth, parent)) = queue.pop_front() {
        if depth > max_depth {
            continue;
        }
        
        // Record this node occurrence
        let node = LinkNode {
            path: current.clone(),
            depth,
            children: Vec::new(),
        };
        
        // Add to parent's children
        if let Some(parent_path) = parent {
            if let Some(parent_node) = result_map.get_mut(&parent_path) {
                parent_node.children.push(node);
            }
        } else {
            // Root node
            result_map.insert(current.clone(), node);
        }
        
        // Only traverse children if not visited AND not at max depth
        if !visited.contains(&current) && depth < max_depth {
            visited.insert(current.clone());
            
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
    
    result_map.into_values().collect()
}
```

### Implementation Steps

1. **Add `--depth` flag** to CLI (`src/main.rs`)
   ```rust
   Links {
       note: String,
       forward: bool,
       backward: bool,
       json: bool,
       #[arg(long, default_value_t = 1)]
       depth: usize,
   }
   ```

2. **Create traversal module** (`src/links/traverse.rs`)
   - `LinkNode` struct
   - `traverse_links()` function
   - Unit tests with cycles

3. **Update output formatting** (`src/commands/links.rs`)
   - `output_tree()` for text format with tree drawing
   - `output_nested_json()` for nested JSON
   - Use traversal results instead of direct index lookups

4. **Add tree drawing utilities**
   - Helper function for `├──`, `└──`, `│` prefixes
   - Track whether node is last sibling

5. **Test with cycles**
   - Create fixture vault with circular references
   - Verify visited set prevents infinite loops
   - Verify nodes appear multiple times via different paths

### Edge Cases to Handle

**Self-loops:**
```
A → A
```
Result: A appears twice (root + child), but second occurrence doesn't expand.

**Diamond pattern:**
```
A → B → D
A → C → D
```
Result: D appears twice (two paths), children expanded only once.

**Deep cycle:**
```
A → B → C → A
```
With depth 3: Shows A→B→C→A, but A's children not re-expanded.

**Orphan (no links):**
```
A (no forward or backward links)
```
Result: Just shows the note itself, no tree.

### Testing Strategy

**Unit tests** (`src/links/traverse.rs`):
- Simple chain (A→B→C)
- Diamond pattern
- Self-loop
- Deep cycle
- Max depth cutoff

**Integration tests** (`tests/links.rs`):
- Build index on fixture vault with cycles
- Query with various depths
- Verify tree output format
- Verify JSON structure
- Verify cycle prevention

---

## Files to Modify

1. `src/main.rs` - Add `depth` parameter to `Links` command
2. `src/commands/mod.rs` - Pass `depth` to handler
3. `src/commands/links.rs` - Use traversal instead of direct lookup, implement tree output
4. `src/links/mod.rs` - Export `traverse` module
5. `src/links/traverse.rs` - **NEW FILE** - Traversal algorithm
6. `tests/fixtures/vault/` - Add notes with circular references
7. `tests/links.rs` - **NEW FILE** - Integration tests

---

## Reference

See `plan/link-index.md` for:
- Detailed traversal algorithm
- Complete output format examples
- All edge cases and behavior
