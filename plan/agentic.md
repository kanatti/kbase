# kbase for Coding Agents

**Purpose**: Define the most important features that make kbase valuable for AI coding agents (Claude, GPT, Pi, etc.)  
**Status**: Planning / Prioritization  
**Date**: 2026-02-22

---

## Agent Workflows (What Agents Actually Need)

### 1. **Context Gathering** 
"I need information about X to complete my task"

**Current Pain**: Agent must manually search, read multiple notes, follow links, assemble context
**What Agents Need**: One command that returns all relevant information, pre-assembled and ranked

### 2. **Knowledge Exploration**
"What does this knowledge base contain? What topics are covered?"

**Current Pain**: No quick overview, must list domains/notes manually
**What Agents Need**: Concept map, topic coverage, entry points for learning

### 3. **Dependency Discovery**
"What do I need to understand first before diving into this topic?"

**Current Pain**: No reading order, no prerequisite graph
**What Agents Need**: "Read these first" recommendations based on links and complexity

### 4. **Impact Analysis**
"If I need to understand/change X, what else is related?"

**Current Pain**: Can't see what links to a note (backlinks)
**What Agents Need**: Bidirectional link graph, dependency trees

### 5. **Knowledge Validation**
"Does the knowledge base have information about Y?"

**Current Pain**: Boolean search only, no semantic understanding
**What Agents Need**: Concept-aware search, "do you have info about BKD trees?"

### 6. **Gap Detection**
"What information is missing or incomplete?"

**Current Pain**: No way to identify coverage gaps
**What Agents Need**: Unresolved links, mentioned but undocumented concepts

---

## Feature Priority Matrix

### Tier 1: Essential (Implement First)

These features solve the most common agent workflows and provide foundational capabilities.

#### **1.1 Link Index & Backlinks**
- **Workflow**: Impact Analysis, Context Gathering
- **Commands**:
  ```bash
  kbase index --only links
  kbase links <note>              # outgoing links
  kbase backlinks <note>          # incoming links (who references this?)
  kbase links validate            # find broken links
  ```
- **Why Critical**: Agents need to understand relationships between notes. Backlinks are especially important - "what else discusses this topic?"
- **Implementation**: Straightforward, builds on existing wikilink parsing
- **Data**: `~/.kbase/<vault>/links.json`

#### **1.2 Context Expansion**
- **Workflow**: Context Gathering
- **Commands**:
  ```bash
  kbase read <note> --expand-links              # include linked content
  kbase read <note> --expand-links --depth 2    # recursive expansion
  kbase context <note>                          # bidirectional context
  ```
- **Why Critical**: Killer feature for LLMs - auto-assemble relevant context without manual hunting
- **Implementation**: Use link index, traverse graph, dedup, format output
- **Options**: JSON format, max tokens, depth limit, circular detection

#### **1.3 Full-Text Search (Tantivy)**
- **Workflow**: Context Gathering, Knowledge Validation
- **Commands**:
  ```bash
  kbase search "BKD tree indexing"
  kbase search "BKD tree" --domain lucene
  kbase search "compression" --format snippets  # return excerpts, not paths
  ```
- **Why Critical**: Agents need to find information by keywords, not just navigate structure
- **Implementation**: Build Tantivy index in `kbase index`, query with BM25 ranking
- **Schema**: path, domain, title (2× boost), content
- **Output**: Ranked results with scores, snippet extraction

#### **1.4 Tag-Based Context Filtering**
- **Workflow**: Context Gathering, Knowledge Exploration
- **Commands**:
  ```bash
  kbase notes --tag deep-dive --tag rust        # intersection
  kbase notes --tag wip --domain lucene         # tag + domain
  kbase context <note> --follow-tags deep-dive  # only expand to notes with tag
  ```
- **Why Critical**: Tags categorize content - agents can filter by topic/status/type
- **Implementation**: Already partially done, enhance with tag intersection/union
- **Enhancement**: Tag hierarchies (`#rust/async`, `#rust/macros`)

---

### Tier 2: High Value (Implement After Tier 1)

These features significantly improve agent capabilities but depend on Tier 1 foundations.

#### **2.1 Smart Context Assembly**
- **Workflow**: Context Gathering
- **Commands**:
  ```bash
  kbase bundle <note>                           # auto-assemble context
  kbase bundle <note> --strategy breadth-first
  kbase bundle <note> --max-tokens 10000
  kbase bundle <note> --format json
  ```
- **Why Valuable**: Goes beyond simple expansion - uses heuristics to build optimal context
- **Strategies**:
  - Breadth-first: Overview before details
  - Depth-first: Follow one path fully
  - Importance-weighted: Use backlink count, centrality scores
  - Tag-guided: Prefer notes with matching tags
- **Output Formats**:
  ```json
  {
    "root": "lucene/search-flow.md",
    "notes": [
      {
        "path": "lucene/search-flow.md",
        "title": "Search Flow Deep Dive",
        "depth": 0,
        "importance": 0.95,
        "content": "..."
      },
      ...
    ],
    "stats": {
      "total_tokens": 8432,
      "notes_included": 5,
      "notes_excluded": 2
    }
  }
  ```

#### **2.2 Semantic Search (Hybrid)**
- **Workflow**: Knowledge Validation, Context Gathering
- **Commands**:
  ```bash
  kbase search "how does lucene handle high cardinality fields?" --semantic
  kbase search "query optimization" --hybrid --top 10
  ```
- **Why Valuable**: Agents ask questions in natural language, not just keywords
- **Implementation**:
  - Use `nomic-embed-text` (local model, 8192 context, 768 dims)
  - Build embedding index during `kbase index` (per-paragraph)
  - Hybrid search: combine semantic similarity + BM25
  - Re-rank results using RRF (Reciprocal Rank Fusion)
- **Storage**: `~/.kbase/<vault>/embeddings.db` (SQLite with vector ext) or FAISS

#### **2.3 Knowledge Graph Queries**
- **Workflow**: Dependency Discovery, Knowledge Exploration
- **Commands**:
  ```bash
  kbase graph path <note-a> <note-b>            # find connection
  kbase graph neighborhood <note> --depth 2     # notes within N hops
  kbase graph hubs --min-links 5                # highly connected notes
  kbase graph clusters --min-size 3             # identify topic clusters
  kbase graph orphans                           # isolated notes
  ```
- **Why Valuable**: Understand knowledge structure, discover relationships
- **Implementation**: Graph algorithms on link index (BFS, connected components, centrality)

#### **2.4 Gap Detection**
- **Workflow**: Gap Detection, Knowledge Validation
- **Commands**:
  ```bash
  kbase gaps                                    # all gaps
  kbase gaps --domain lucene
  kbase gaps --unresolved-links                 # broken wikilinks
  kbase gaps --mentioned-concepts               # text mentions without notes
  ```
- **Why Valuable**: Helps agents know what info is missing, guides knowledge base expansion
- **Implementation**:
  - Track unresolved wikilinks during indexing
  - Extract frequently mentioned concepts (NER-style)
  - Compare against existing note titles/tags
  - Report gaps with frequency counts

#### **2.5 Tag Analytics & Suggestions**
- **Workflow**: Knowledge Exploration, Gap Detection
- **Commands**:
  ```bash
  kbase tags --stats                            # coverage by tag
  kbase tags suggest <note>                     # auto-suggest tags
  kbase tags hierarchy                          # show tag tree
  ```
- **Why Valuable**: Better organization, discoverability
- **Implementation**:
  - Tag co-occurrence analysis
  - Content similarity → suggest tags used by similar notes
  - Support tag hierarchies (`#rust/async` → parent: `#rust`)
- **Tag Namespaces**: `#domain/topic/subtopic` for hierarchical organization

---

### Tier 3: Advanced Features (Future)

These are valuable for specialized use cases but not critical for basic agent workflows.

#### **3.1 Concept Extraction & Index**
- **Commands**:
  ```bash
  kbase concepts                                # list all concepts
  kbase concepts --domain lucene
  kbase concept "BKD tree" --definition         # extract definition
  kbase concept "BKD tree" --notes              # where is this discussed?
  ```
- **Implementation**: NER + definition extraction, concept → notes mapping

#### **3.2 Note Summarization**
- **Commands**:
  ```bash
  kbase summarize <note>
  kbase summarize <note> --extract facts
  kbase summarize <note> --extract code-snippets
  ```
- **Implementation**: LLM-powered summarization with structured output

#### **3.3 Dependency Graph (Prerequisites)**
- **Commands**:
  ```bash
  kbase prerequisites <note>                    # what to read first
  kbase reading-order --topic "lucene search"   # suggested path
  ```
- **Implementation**: Analyze links + complexity heuristics + naming conventions

#### **3.4 Multi-Domain Research**
- **Commands**:
  ```bash
  kbase compare lucene datafusion --concept "columnar compression"
  ```
- **Implementation**: Cross-domain concept extraction and comparison

#### **3.5 Section-Level Targeting**
- **Commands**:
  ```bash
  kbase read <note>#<section>                   # specific section only
  kbase links validate --check-sections         # validate heading targets
  ```
- **Implementation**: Parse `[[note#section]]`, resolve heading anchors, extract section content

#### **3.6 Auto-Generated Indexes**
- **Commands**:
  ```bash
  kbase auto-index --domain lucene              # generate overview
  kbase auto-index --tag deep-dive
  ```
- **Implementation**: Template-based index generation with link graphs, summaries

---

## Implementation Roadmap

### Phase 1: Foundation (Essential for agents)
**Goal**: Enable basic agent workflows - context gathering, link navigation, search

- [ ] **Link Index** - Build backlink/forward link mapping
  - `kbase index --only links` 
  - `kbase links <note>`
  - `kbase backlinks <note>`
  - `kbase links validate`

- [ ] **Tantivy Search** - Full-text search with ranking
  - Build index during `kbase index`
  - `kbase search <query>`
  - `kbase search <query> --domain <domain>`
  - Snippet extraction from results

- [ ] **Context Expansion** - Auto-assemble linked content
  - `kbase read <note> --expand-links`
  - `kbase read <note> --expand-links --depth N`
  - `kbase context <note>` (bidirectional)
  - JSON output format

- [ ] **Tag Enhancements**
  - Tag intersection: `kbase notes --tag A --tag B`
  - Tag hierarchies: `#rust/async` support
  - Context filtering: `kbase context <note> --follow-tags <tag>`

**Deliverable**: Agents can search, navigate links, and assemble context automatically

---

### Phase 2: Intelligence (High-value enhancements)
**Goal**: Semantic understanding, smart assembly, graph queries

- [ ] **Semantic Search** - Embeddings + hybrid search
  - Build embedding index (per-paragraph)
  - `kbase search <query> --semantic`
  - Hybrid ranking (BM25 + cosine similarity)

- [ ] **Smart Context Assembly** - Heuristic-based bundling
  - `kbase bundle <note>`
  - Multiple strategies (breadth/depth/importance)
  - Token budgeting
  - JSON output with metadata

- [ ] **Knowledge Graph Queries**
  - `kbase graph path <a> <b>`
  - `kbase graph neighborhood <note>`
  - `kbase graph hubs/clusters/orphans`

- [ ] **Gap Detection**
  - `kbase gaps`
  - Unresolved links, mentioned concepts
  - Frequency-based ranking

**Deliverable**: Agents have intelligent context assembly and gap awareness

---

### Phase 3: Advanced (Specialized features)
**Goal**: Sophisticated analysis and automation

- [ ] Concept extraction and indexing
- [ ] Note summarization (LLM-powered)
- [ ] Prerequisite detection
- [ ] Multi-domain comparison
- [ ] Section-level targeting
- [ ] Auto-generated indexes

**Deliverable**: Advanced agent capabilities for complex workflows

---

## Technical Architecture

### Index Storage
```
~/.kbase/<vault-name>/
  tags.json              # tag → [paths] (already exists)
  links.json             # source → [targets] (new)
  search.tantivy/        # Tantivy full-text index (new)
  embeddings.db          # SQLite with vectors (phase 2)
  concepts.json          # concept → [paths, definition] (phase 3)
```

### Data Structures

**Link Index**:
```rust
struct LinkIndex {
    // Forward links: source → targets
    forward: HashMap<PathBuf, Vec<LinkTarget>>,
    
    // Backlinks: target → sources (derived from forward)
    backlinks: HashMap<PathBuf, Vec<LinkSource>>,
}

struct LinkTarget {
    path: PathBuf,
    section: Option<String>,
    line: usize,
}

struct LinkSource {
    path: PathBuf,
    line: usize,
    alias: Option<String>,
}
```

**Tantivy Schema**:
```rust
schema! {
    path: TEXT, stored, not_indexed,
    domain: TEXT, stored, indexed,
    title: TEXT, stored, indexed, boost=2.0,
    content: TEXT, not_stored, indexed,
}
```

**Embedding Index** (phase 2):
```sql
CREATE TABLE embeddings (
    note_path TEXT,
    paragraph_idx INTEGER,
    paragraph_text TEXT,
    embedding BLOB,  -- 768 floats (nomic-embed)
    PRIMARY KEY (note_path, paragraph_idx)
);
```

### Context Expansion Algorithm

```rust
fn expand_context(
    root: &Path,
    depth: usize,
    max_tokens: Option<usize>,
    strategy: Strategy,
) -> ExpandedContext {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut notes = Vec::new();
    
    queue.push_back((root, 0));
    
    while let Some((path, d)) = queue.pop_front() {
        if visited.contains(path) || d > depth {
            continue;
        }
        
        // Read note, parse, add to result
        let note = read_note(path)?;
        notes.push(note);
        visited.insert(path);
        
        // Check token budget
        if let Some(max) = max_tokens {
            if total_tokens(&notes) > max {
                break;
            }
        }
        
        // Enqueue linked notes based on strategy
        for link in get_links(path, strategy) {
            queue.push_back((link, d + 1));
        }
    }
    
    ExpandedContext { root, notes }
}
```

### Hybrid Search (Phase 2)

```rust
fn hybrid_search(
    query: &str,
    k: usize,
) -> Vec<SearchResult> {
    // BM25 search
    let bm25_results = tantivy_search(query, k * 2);
    
    // Semantic search
    let query_embedding = embed_text(query);
    let semantic_results = vector_search(query_embedding, k * 2);
    
    // Reciprocal Rank Fusion
    rrf_merge(bm25_results, semantic_results, k)
}

fn rrf_merge(
    bm25: Vec<Result>,
    semantic: Vec<Result>,
    k: usize,
) -> Vec<Result> {
    let mut scores = HashMap::new();
    
    for (rank, result) in bm25.iter().enumerate() {
        scores.entry(result.path)
            .or_insert(0.0)
            += 1.0 / (60.0 + rank as f32);
    }
    
    for (rank, result) in semantic.iter().enumerate() {
        scores.entry(result.path)
            .or_insert(0.0)
            += 1.0 / (60.0 + rank as f32);
    }
    
    // Sort by RRF score, take top k
    scores.into_iter()
        .sorted_by(|a, b| b.1.partial_cmp(&a.1).unwrap())
        .take(k)
        .collect()
}
```

---

## Output Formats for Agents

### JSON Format (Structured)

**Search Results**:
```json
{
  "query": "BKD tree indexing",
  "total": 42,
  "top_k": 5,
  "results": [
    {
      "path": "lucene/bkd-trees.md",
      "title": "BKD Trees Deep Dive",
      "score": 0.95,
      "snippet": "...BKD trees use a space-partitioning approach...",
      "domain": "lucene",
      "tags": ["deep-dive", "indexing"]
    }
  ]
}
```

**Expanded Context**:
```json
{
  "root": "lucene/search-flow.md",
  "strategy": "breadth-first",
  "depth": 2,
  "notes": [
    {
      "path": "lucene/search-flow.md",
      "title": "Search Flow Deep Dive",
      "depth": 0,
      "content": "...",
      "links_to": ["lucene/codecs.md"],
      "linked_from": ["lucene/01-home.md"],
      "tags": ["deep-dive"]
    }
  ],
  "stats": {
    "total_notes": 5,
    "total_tokens": 8432,
    "depth_reached": 2
  }
}
```

### Text Format (Readable)

**Concatenated Context**:
```
=== Context for: lucene/search-flow.md ===
Depth: 2 | Strategy: breadth-first | Total: 5 notes | Tokens: 8432

--- lucene/search-flow.md (depth: 0) ---
# Search Flow Deep Dive
...

--- lucene/codecs.md (depth: 1, linked from: lucene/search-flow.md) ---
# Codecs
...
```

---

## Agent Integration Examples

### Example 1: Claude Desktop / Pi
```bash
# Agent needs context about Lucene search implementation
kbase bundle lucene/search-flow.md --format json --max-tokens 10000

# Agent receives JSON with:
# - Main note + linked notes
# - Metadata (tags, links, depth)
# - Total token count
# Agent can now answer questions about search flow
```

### Example 2: Cursor / Codebase Agent
```bash
# Agent working on DataFusion, needs to check if there's info about BKD trees
kbase search "BKD tree" --semantic --format json

# Returns:
# - Ranked results with scores
# - Snippets showing relevant context
# - Paths for full content retrieval
# Agent decides to read lucene/bkd-trees.md
```

### Example 3: Research Assistant
```bash
# Agent exploring Lucene domain
kbase graph neighborhood lucene/01-home.md --depth 2 --format json

# Returns:
# - Graph structure (nodes, edges)
# - Important notes (high centrality)
# - Suggested reading order
# Agent builds understanding of domain structure
```

### Example 4: Context Assembly
```bash
# Agent needs broad context about compression across domains
kbase search "compression" --domain lucene --format snippets
kbase search "compression" --domain datafusion --format snippets
kbase compare lucene datafusion --concept compression

# Agent gets:
# - Relevant snippets from both domains
# - Comparative analysis
# - Related notes for deeper dive
```

---

## Success Metrics

How do we know these features are valuable for agents?

1. **Context Quality**: Can agents answer complex questions using assembled context?
2. **Efficiency**: Tokens used per query (less is better with good context)
3. **Coverage**: % of agent queries that find relevant information
4. **Navigation**: Steps to find info (1 command vs 5+ manual steps)
5. **Discovery**: Can agents find information they didn't know to ask about?

---

## Related Documents

- `plan/wikilinks-agentic.md` - Detailed wikilink feature brainstorm
- `plan/index.md` - Index architecture and storage
- `plan/ideas.md` - Additional innovation ideas
- `docs/tree-sitter.md` - Wikilink parsing implementation
- `docs/tags.md` - Current tag system
