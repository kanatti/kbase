# Link Index Implementation Guide

**Date**: 2026-02-22  
**Status**: Ready to implement  
**Context**: Quick reference for implementation details

---

## Algorithm: Single Pass Bidirectional Index Build

```rust
// Single pass through all notes
for source_file in all_notes {
    let wikilinks = parse(source_file);
    
    for wikilink in wikilinks {
        // Filter out non-markdown links
        if !should_index_wikilink(&wikilink.target) {
            continue;
        }
        
        // Resolve wikilink string to actual path
        if let Some(target) = resolve(wikilink.target, source_file) {
            // Populate BOTH maps simultaneously
            forward[source_file].push(target);
            backward[target].push(source_file);
        }
    }
}
```

**Key insight**: No need to build forward then invert. Build both in one pass!

---

## Wikilink Types in kanatti-notes Vault

Based on actual vault exploration:

### 1. Bare Names (no path prefix)
```markdown
[[01-home]]
[[indexed-disi]]
[[direct-writer]]
```
**Resolution**: Try same-domain first, then stem lookup across all domains

### 2. Path-Style (domain/note)
```markdown
[[delta/02-logs]]
[[lucene/numeric-doc-values]]
[[books/the-intelligent-investor]]
```
**Resolution**: Must match exact path, no fallback

### 3. With Aliases
```markdown
[[indexed-disi|IndexedDISI]]
[[delta/strategy|Strategies]]
[[sharpe-ratio|Sharpe Ratio]]
```
**Handling**: Alias discarded during indexing (only in `Wikilink.alias` field)

### 4. With Sections
```markdown
[[cmu-database-systems-course#Database Storage]]
[[backlog#someday-maybe|Someday/Maybe]]
```
**Handling**: Parser already strips section! `Wikilink.target` has section removed.
- Input: `[[note#section]]`
- Parser gives: `Wikilink { target: "note", section: Some("section"), ... }`
- We use: `wikilink.target` (section already stripped)

### 5. Images (filter out)
```markdown
[[Screenshot 2025-10-19 at 11.11.10 AM.png]]
[[parquet-row-groups.png]]
```
**Handling**: Skip during indexing

### 6. Multiple on One Line
```markdown
[[01-home]] | [[delta/02-logs]] | [[delta/04-archive]]
```
**Handling**: Parser extracts each separately, works automatically

---

## Filtering Logic

**Accept for indexing:**
- No extension: `[[note]]` → assumes markdown
- `.md` extension: `[[note.md]]` → explicit markdown

**Reject:**
- Non-md extensions: `[[image.png]]`, `[[diagram.svg]]`

```rust
fn should_index_wikilink(target: &str) -> bool {
    // target already has section stripped by parser
    
    // Check if it has an extension
    if let Some(ext_start) = target.rfind('.') {
        let ext = &target[ext_start + 1..];
        ext == "md"  // Only accept .md
    } else {
        true  // No extension = markdown note
    }
}
```

---

## Parser Output (from tree-sitter)

The parser already does section handling for us:

```rust
// Input wikilinks:
"[[note#Introduction]]"
"[[folder/note#Overview]]"  
"[[note]]"

// Parser produces Wikilink structs:
Wikilink { target: "note", section: Some("Introduction"), ... }
Wikilink { target: "folder/note", section: Some("Overview"), ... }
Wikilink { target: "note", section: None, ... }
```

**Just use `wikilink.target` for resolution!** Section is already stripped.

---

## Resolution Strategy (Simple, No Magic)

```rust
fn resolve_target(
    target: &str,
    source_path: &Path,
    all_notes: &HashSet<PathBuf>,
) -> Option<PathBuf>
```

### Path-style (`target` contains `/`)
- Example: `"lucene/codecs"`, `"internals/codec"`
- Try absolute from vault root: `target.md`
- Try relative to source domain: `source_domain/target.md`
- If not found: **fail** (no fallback)

### Bare name (`target` has no `/`)
- Example: `"codecs"`, `"glossary"`
- Try same-domain: `source_domain/target.md`
- Try root level: `target.md`
- If not found: **fail**

### No Cross-Domain Magic!
- `lucene/test.md` → `[[glossary]]` → `glossary.md` ✅ (root level OK)
- `lucene/test.md` → `[[query-execution]]` → ❌ (not in lucene/, not at root)
- Want cross-domain? Use explicit path: `[[datafusion/query-execution]]` ✅

**Why no vault-wide stem lookup?**
- Avoids "magic" resolution that's hard to predict
- Root-level notes are special (global scope) - that's intentional
- Other domains should require explicit paths
- Simpler implementation, easier to understand

---

## Data Structures

### During Indexing
```rust
struct LinkIndexBuilder {
    all_notes: HashSet<PathBuf>,             // All note paths in vault
    forward: HashMap<PathBuf, Vec<PathBuf>>, // source → targets
    backward: HashMap<PathBuf, Vec<PathBuf>>, // target → sources
    unresolved_count: usize,
}
```

### On Disk (JSON)
```rust
// Both files: PathBuf as string → Vec<PathBuf> as strings
type LinkMap = HashMap<String, Vec<String>>;
```

### After Loading
```rust
struct LinkIndex {
    forward: HashMap<PathBuf, Vec<PathBuf>>,
    backward: HashMap<PathBuf, Vec<PathBuf>>,
}
```

---

## Implementation Steps (Indexing Only)

1. **Create `src/links/` module**
   - `mod.rs` - public API
   - `index.rs` - LinkIndex struct and building logic
   - `resolve.rs` - resolution algorithm

2. **Implement resolution** (`resolve.rs`)
   - `resolve_target()` function
   - Handle path-style vs bare name
   - Same-domain priority, stem lookup fallback

3. **Implement index building** (`index.rs`)
   - `LinkIndex::build_from_vault()` 
   - Single pass: parse → filter → resolve → populate both maps
   - Sort and dedup path arrays
   - `save_to_json()` with atomic write

4. **Hook up to `kbase index`** (`commands/index.rs`)
   - Build link index when `--only links` or no `--only` flag

5. **Test with fixture vault**
   - Unit tests for resolution
   - Integration test for full index build

---

## Progress

✅ **Resolution logic complete** (`src/links/resolve.rs`)
- Simple, predictable rules (no cross-domain magic)
- 10 comprehensive tests
- Path-style and bare name handling
- Root level fallback for bare names

## Next Session Resume Point

Implement `LinkIndex::build_from_vault()` in `src/links/index.rs`

Reference `src/tags/index.rs` as the pattern to follow.
