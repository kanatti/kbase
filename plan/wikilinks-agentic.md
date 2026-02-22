# Wikilinks for Agentic Use Cases

**Status**: Brainstorming  
**Date**: 2026-02-22  
**Context**: We have wikilink parsing via tree-sitter but haven't built features on top of it yet.

## Current State

### What We Parse
- `[[note]]` - Basic link to a note
- `[[note|alias]]` - Link with display text
- `[[domain/note]]` - Link with domain/path
- `[[note#section]]` - Link to a heading within a note
- `[[domain/note#section|Custom Text]]` - All features combined

### Extraction Capabilities
The `Wikilink` struct gives us:
```rust
pub struct Wikilink {
    pub target: String,          // "domain/note" or "note"
    pub alias: Option<String>,   // Display text
    pub section: Option<String>, // Heading anchor
    pub line: usize,             // Position in source
    pub column: usize,
}
```

## Agentic Use Cases

### 1. Context Navigation
**Problem**: Coding agents need to gather relevant context across notes.

**Features**:
- **Follow links**: From note A, discover what it references
- **Backlinks**: From note A, discover what references it
- **Bi-directional graph**: Traverse connections in both directions
- **Context expansion**: "Give me this note plus all notes it references"

**Commands**:
```bash
kbase links <note>                    # Show outgoing links
kbase backlinks <note>                # Show incoming links
kbase related <note>                  # Both directions
kbase context <note> --depth 2        # Expand context N hops
```

### 2. Graph Traversal & Discovery
**Problem**: Agents need to understand knowledge structure and relationships.

**Features**:
- **Neighborhood queries**: Find notes within N hops
- **Path finding**: Find connection between two notes
- **Clustering**: Identify densely connected note groups
- **Orphan detection**: Notes with no links (in or out)
- **Hub identification**: Notes with many connections

**Commands**:
```bash
kbase graph neighborhood <note> --depth 2
kbase graph path <note-a> <note-b>
kbase graph clusters --min-size 3
kbase graph orphans
kbase graph hubs --min-links 5
```

### 3. Link Index & Validation
**Problem**: Broken links, missing targets, structural issues.

**Features**:
- **Build link index**: Map target → sources (backlinks)
- **Validate links**: Check for broken references
- **Suggest fixes**: Fuzzy match for typos
- **Link health report**: Dashboard of link quality

**Commands**:
```bash
kbase index --only links              # Build link index
kbase links validate                  # Check all links
kbase links broken                    # List broken links
kbase links suggest <broken-link>     # Fuzzy match suggestions
```

### 4. Smart Context Retrieval
**Problem**: LLMs need relevant context without manual hunting.

**Features**:
- **Auto-expand**: Resolve wikilinks and include content
- **Depth control**: How many hops to expand
- **Circular detection**: Avoid infinite loops
- **Token budgeting**: Stop when token limit reached
- **Relevance filtering**: Only expand links matching criteria

**Commands**:
```bash
kbase read <note> --expand-links              # Include linked content
kbase read <note> --expand-links --depth 2    # Recursive expansion
kbase read <note> --expand-links --max-tokens 10000
kbase context <note> --tag deep-dive          # Only expand links to notes with tag
```

### 5. Section-Aware Navigation
**Problem**: Links point to specific sections, not whole notes.

**Features**:
- **Resolve sections**: `[[note#Introduction]]` → exact heading
- **Section validation**: Check if target heading exists
- **Section context**: Return only relevant section + N lines
- **Heading anchors**: Support Obsidian-style `#heading-slug` format

**Commands**:
```bash
kbase read <note>#<section>                   # Read specific section
kbase links validate --check-sections         # Validate heading targets
kbase read <note> --expand-sections           # Expand section links inline
```

### 6. Link-Based Search & Filtering
**Problem**: Discover notes through link patterns, not just tags/text.

**Features**:
- **Citing search**: "What notes link to X?"
- **Referenced by**: "What does X reference?"
- **Co-citation**: "What notes link to both X and Y?"
- **Link count ranking**: Sort by popularity/importance
- **Unlinked mentions**: Find text mentions without actual links

**Commands**:
```bash
kbase notes --links-to <note>                 # What cites this?
kbase notes --links-from <note>               # What does this cite?
kbase notes --links-both <note-a> <note-b>    # Co-citation
kbase notes --sort links                      # By link count
kbase links mentions <note>                   # Find unlinked mentions
```

### 7. Dependency Tracking
**Problem**: Notes have implicit dependencies through links.

**Features**:
- **Dependency tree**: Show what a note depends on (outgoing)
- **Dependent tree**: Show what depends on a note (incoming)
- **Change impact**: "If I edit this, what's affected?"
- **Topological sort**: Order notes by dependency

**Commands**:
```bash
kbase deps <note>                             # Show dependencies
kbase deps <note> --dependents                # Show dependents
kbase deps <note> --impact                    # Full impact tree
kbase deps graph --sort                       # Topological order
```

### 8. Context Assembly for Agents
**Problem**: Agents need curated context bundles, not manual assembly.

**Features**:
- **Smart bundles**: Auto-assemble related notes
- **Link-guided retrieval**: Follow semantic connections
- **Deduplication**: Include each note once
- **Priority ordering**: Most important notes first
- **Format options**: JSON, concatenated text, tree structure

**Commands**:
```bash
kbase bundle <note>                           # Auto-assemble context
kbase bundle <note> --format json             # Structured output
kbase bundle <note> --strategy breadth-first  # Traversal strategy
kbase bundle <note> --priority backlinks      # Prioritize by backlinks
```

### 9. Link Metrics & Analytics
**Problem**: Understanding knowledge base structure and quality.

**Features**:
- **Link density**: Links per note average
- **Degree distribution**: Connection patterns
- **Centrality scores**: Identify important notes (PageRank-style)
- **Clustering coefficient**: How interconnected are neighborhoods?
- **Weakly connected components**: Isolated clusters

**Commands**:
```bash
kbase stats links                             # Overall link metrics
kbase graph centrality                        # Rank by importance
kbase graph components                        # Find isolated clusters
kbase graph density --domain <domain>         # Domain-specific metrics
```

### 10. Interactive Graph Exploration
**Problem**: Visual understanding of knowledge structure.

**Features**:
- **ASCII graph rendering**: Terminal-friendly visualization
- **Filtered views**: Show subgraphs (domain, tag, depth)
- **Highlight paths**: Show route between notes
- **Export formats**: DOT, JSON, CSV for external tools

**Commands**:
```bash
kbase graph show <note> --depth 2             # ASCII visualization
kbase graph show <note> --format dot > graph.dot
kbase graph export --domain lucene            # Full domain graph
```

## Implementation Priorities

### Phase 1: Basic Link Index (Essential for everything else)
```bash
kbase index --only links
kbase links <note>         # Show outgoing links from note
kbase backlinks <note>     # Show incoming links to note
kbase links validate       # Check for broken links
```

**Data Structure**:
```rust
struct LinkIndex {
    // target -> sources (backlinks)
    backlinks: HashMap<String, Vec<LinkReference>>,
    
    // source -> targets (forward links)
    forward_links: HashMap<String, Vec<Wikilink>>,
    
    // All unique link targets (for validation)
    all_targets: HashSet<String>,
}

struct LinkReference {
    source: PathBuf,
    line: usize,
    column: usize,
    alias: Option<String>,
}
```

### Phase 2: Context Expansion (High value for agents)
```bash
kbase read <note> --expand-links
kbase context <note> --depth 2
kbase bundle <note>
```

This is the killer feature for LLM agents - auto-assembling relevant context.

### Phase 3: Validation & Health
```bash
kbase links broken
kbase links suggest <broken-link>
kbase graph orphans
```

Keep the knowledge base healthy and navigable.

### Phase 4: Advanced Queries
```bash
kbase notes --links-to <note>
kbase graph path <a> <b>
kbase graph clusters
```

Power user and advanced agent features.

## Design Considerations

### 1. Link Resolution Strategy
**Question**: How do we resolve `[[note]]` to a file path?

**Options**:
- **Exact match only**: `note.md` must exist in root or be qualified
- **Fuzzy search**: Try to match by similarity (risky)
- **Index-based**: Build a name → path lookup table
- **Obsidian-style**: Check all possible locations, fail if ambiguous

**Recommendation**: Start with exact match, add index-based lookup in Phase 1.

### 2. Circular Reference Handling
**Question**: What happens with `[[A]]` ↔ `[[B]]`?

**Options**:
- **Visited set**: Track expanded notes, skip if seen
- **Depth limit**: Never expand beyond N hops
- **Both**: Depth limit + visited set for safety

**Recommendation**: Both, with clear error messages.

### 3. Performance & Caching
**Question**: Re-parse every time or cache?

**Options**:
- **Always re-parse**: Simple, always fresh, slower
- **Cache parsed metadata**: Fast, stale risk, cache invalidation needed
- **Hybrid**: Cache index, parse on demand

**Recommendation**: Start without cache, add if performance issues arise.

### 4. Section Resolution
**Question**: How to match `[[note#Section Name]]`?

**Options**:
- **Exact heading match**: Must match perfectly (case-sensitive?)
- **Obsidian slug format**: `#heading-slug` normalization
- **Fuzzy heading match**: Similar to heading text
- **Line-based**: Approximate based on heading order

**Recommendation**: Exact match first, then add slug-based matching.

### 5. Output Formats for Agents
**Question**: How should agents consume expanded context?

**Options**:
```json
{
  "root": "lucene/search-flow.md",
  "notes": [
    {
      "path": "lucene/search-flow.md",
      "title": "Search Flow Deep Dive",
      "depth": 0,
      "content": "...",
      "links": ["lucene/codecs.md"]
    },
    {
      "path": "lucene/codecs.md",
      "title": "Codecs",
      "depth": 1,
      "content": "...",
      "referenced_by": ["lucene/search-flow.md"]
    }
  ]
}
```

Or concatenated text with markers:
```
=== lucene/search-flow.md (depth: 0) ===
# Search Flow Deep Dive
...

=== lucene/codecs.md (depth: 1, referenced by: lucene/search-flow.md) ===
# Codecs
...
```

**Recommendation**: Both. JSON for structured consumption, text for quick reading.

## Agent Workflow Examples

### Example 1: Understanding a Complex Topic
```bash
# Agent wants to understand Lucene's search flow
kbase read lucene/search-flow.md --expand-links --depth 2

# Returns:
# - lucene/search-flow.md (main content)
# - lucene/codecs.md (referenced at depth 1)
# - lucene/scoring.md (referenced at depth 1)
# - lucene/index-structure.md (referenced at depth 2)
```

### Example 2: Finding Related Context
```bash
# Agent needs everything related to "BKD trees"
kbase search "BKD trees"              # Find relevant note
kbase context lucene/bkd-trees.md     # Expand context

# Returns bidirectional context:
# - Notes that bkd-trees.md links to (forward)
# - Notes that link to bkd-trees.md (backlinks)
```

### Example 3: Impact Analysis
```bash
# Agent wants to know what would be affected by editing a note
kbase backlinks elasticsearch/esql-analysis.md

# Shows all notes that reference this one
# Agent can then check those notes before making changes
```

### Example 4: Knowledge Discovery
```bash
# Agent explores a domain through link structure
kbase graph neighborhood lucene/01-home.md --depth 3
kbase graph hubs --domain lucene

# Discovers:
# - Central concepts (highly linked notes)
# - Clusters of related notes
# - Entry points for learning
```

## Open Questions

1. **Ambiguous links**: What if `[[note]]` could match multiple files?
   - Return error?
   - Return all matches?
   - Use heuristics (prefer domain context)?

2. **Cross-vault links**: Do we support `[[other-vault:note]]`?
   - Probably not initially
   - Could add as extension

3. **External links**: Handle `[regular markdown](url)` differently?
   - Probably ignore for now
   - Link index is for wikilinks only

4. **Embed syntax**: Obsidian's `![[note]]` for transclusion?
   - Not parsed yet
   - Could be Phase 5 feature

5. **Alias resolution**: Should we index aliases for search?
   - `[[note|Important Concept]]` - can we find by "Important Concept"?
   - Useful for agents, worth considering

6. **Link suggestions**: Auto-suggest links while editing?
   - Out of scope (kbase is read-focused)
   - But could power editor integrations

## Next Steps

1. **Review this doc** - Get feedback on priorities and approach
2. **Implement Phase 1** - Basic link index and validation
3. **Test with real agent workflows** - See what's actually useful
4. **Iterate on context expansion** - The killer feature
5. **Document agent integration patterns** - How to use kbase from LLM tools

---

**Related**:
- `docs/tree-sitter.md` - Wikilink parsing implementation
- `plan/index.md` - Overall indexing strategy
- `src/parser/tree_sitter.rs` - Current wikilink extraction code
