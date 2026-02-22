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

## Multi-Agent Patterns

Some kbase operations are perfect for **sub-agent delegation** - where a main agent spawns specialized sub-agents to explore, then synthesizes their findings.

### Which Features Benefit from Sub-Agents?

#### **1. Knowledge Graph Exploration** (High Value)
**Pattern**: Main agent asks "what's in the lucene domain?", sub-agent explores and reports back

**Why Sub-Agent**:
- Graph traversal is exploratory and recursive
- Sub-agent can follow links autonomously without polluting main agent's context
- Main agent gets a condensed map instead of full content
- Parallel exploration: multiple sub-agents can explore different starting points

**Command**:
```bash
kbase graph explore lucene/01-home.md --depth 3 --format summary
```

**Sub-Agent Output**:
```json
{
  "exploration_root": "lucene/01-home.md",
  "depth_reached": 3,
  "discovered": {
    "total_notes": 15,
    "key_topics": ["search-flow", "codecs", "indexing", "bkd-trees"],
    "clusters": [
      {"topic": "indexing", "notes": 5, "central_note": "lucene/bkd-trees.md"},
      {"topic": "search", "notes": 6, "central_note": "lucene/search-flow.md"}
    ],
    "hubs": [
      {"note": "lucene/codecs.md", "links": 12, "backlinks": 8}
    ]
  },
  "reading_path": [
    "lucene/01-home.md",
    "lucene/indexing-basics.md",
    "lucene/bkd-trees.md"
  ]
}
```

**Main Agent Workflow**:
1. Main agent: "I need to understand Lucene"
2. Spawn sub-agent: "Explore lucene domain, give me a map"
3. Sub-agent: Follows links, builds graph, identifies clusters
4. Sub-agent returns: Condensed summary (500 tokens instead of 20K)
5. Main agent: "Okay, now fetch me lucene/bkd-trees.md" (targeted read)

#### **2. Multi-Domain Research** (High Value)
**Pattern**: Main agent asks "how do Lucene and DataFusion handle compression?", spawns 2 sub-agents in parallel

**Why Sub-Agent**:
- Each domain is independent, can be explored in parallel
- Sub-agents can use specialized prompts per domain
- Results are synthesized by main agent (comparison, contrast)
- Token budget distributed across sub-agents

**Commands**:
```bash
# Sub-agent 1:
kbase search "compression" --domain lucene --format snippets

# Sub-agent 2:
kbase search "compression" --domain datafusion --format snippets
```

**Main Agent Workflow**:
1. Main agent: "Compare compression approaches in Lucene vs DataFusion"
2. Spawn sub-agent-1: "Find Lucene compression info"
3. Spawn sub-agent-2: "Find DataFusion compression info"
4. Both sub-agents work in parallel (2× faster)
5. Main agent receives both reports
6. Main agent synthesizes: "Lucene uses X, DataFusion uses Y, key difference is Z"

#### **3. Gap Detection & Analysis** (Medium Value)
**Pattern**: Main agent working on task, sub-agent runs async gap analysis

**Why Sub-Agent**:
- Gap detection is comprehensive (scans whole vault)
- Can run in background while main agent works
- Results guide future exploration
- Non-blocking operation

**Command**:
```bash
kbase gaps --domain lucene --detailed
```

**Sub-Agent Output**:
```json
{
  "domain": "lucene",
  "gaps_found": 12,
  "high_priority": [
    {
      "type": "unresolved_link",
      "target": "segment-merging",
      "mentioned_in": 5,
      "context": ["search-flow.md", "codecs.md", ...]
    },
    {
      "type": "mentioned_concept",
      "concept": "index warming",
      "occurrences": 7,
      "should_document": true
    }
  ],
  "recommendations": [
    "Create note for 'segment-merging' (referenced 5 times)",
    "Document 'index warming' concept"
  ]
}
```

**Main Agent Workflow**:
1. Main agent: Working on feature implementation
2. Spawn sub-agent: "Analyze knowledge gaps in lucene domain"
3. Main agent: Continues working (non-blocking)
4. Sub-agent: Scans vault, identifies gaps, returns report
5. Main agent: Reviews gaps when convenient, decides what to document

#### **4. Context Assembly with Strategy Comparison** (Medium Value)
**Pattern**: Main agent unsure which context strategy is best, sub-agents try different approaches

**Why Sub-Agent**:
- Different strategies yield different context
- Sub-agents can try multiple approaches in parallel
- Main agent selects best result based on task
- Exploration vs exploitation trade-off

**Commands**:
```bash
# Sub-agent 1: Breadth-first
kbase bundle lucene/search-flow.md --strategy breadth-first --max-tokens 5000

# Sub-agent 2: Importance-weighted
kbase bundle lucene/search-flow.md --strategy importance --max-tokens 5000

# Sub-agent 3: Tag-guided
kbase bundle lucene/search-flow.md --follow-tags deep-dive --max-tokens 5000
```

**Main Agent Workflow**:
1. Main agent: "I need context about search-flow but not sure what's relevant"
2. Spawn 3 sub-agents with different strategies
3. Sub-agents assemble context independently
4. Main agent receives 3 different context bundles
5. Main agent evaluates: "Breadth-first gave best overview, use that"

#### **5. Semantic Search with Query Expansion** (Low-Medium Value)
**Pattern**: Sub-agent reformulates query, tries variations, finds best results

**Why Sub-Agent**:
- Query reformulation is iterative
- Can try multiple search strategies
- Sub-agent can evaluate which queries yield best results
- Returns consolidated, ranked results

**Sub-Agent Workflow**:
```python
# Sub-agent internal logic
queries = [
    "BKD tree indexing",
    "block KD tree spatial indexing", 
    "multidimensional point indexing lucene"
]

results = []
for q in queries:
    r = kbase_search(q, semantic=True)
    results.extend(r)

# Deduplicate, re-rank, return top K
return dedupe_and_rank(results, k=10)
```

#### **6. Dependency Path Finding** (Low Value, but interesting)
**Pattern**: Sub-agent finds multiple paths between notes, main agent selects best

**Why Sub-Agent**:
- Path finding can be complex (multiple routes)
- Sub-agent can evaluate path quality (shortest, most relevant, etc.)
- Returns ranked paths for main agent to choose

**Command**:
```bash
kbase graph paths lucene/01-home.md lucene/bkd-trees.md --all
```

---

### Sub-Agent Feature Requirements

To support multi-agent workflows well, kbase needs:

#### **1. Structured Output (JSON)**
- All commands should support `--format json`
- Consistent schema across commands
- Metadata included (timing, token counts, confidence)

#### **2. Summary/Condensed Modes**
- `--summary` flag: Return overview, not full content
- Token budgets: `--max-tokens N`
- Snippet extraction: `--format snippets`

Example:
```bash
kbase graph explore <note> --summary
# Returns: Topic map, key notes, reading path (500 tokens)
# Instead of: Full content of all discovered notes (20K tokens)
```

#### **3. Parallel-Friendly Commands**
- Stateless: No side effects, safe to run in parallel
- Read-only: All queries are reads (index is pre-built)
- Fast: Sub-second response for most queries

#### **4. Confidence/Quality Scores**
- Search results: BM25 scores, semantic similarity
- Link relevance: Backlink count, centrality scores
- Gap priority: Mention frequency, impact score

This helps main agent decide which sub-agent results to trust/use.

#### **5. Streaming/Incremental Output** (Future)
For long-running sub-agent tasks:
```bash
kbase graph explore <note> --depth 5 --stream
# Outputs discoveries incrementally as JSON lines
# Main agent can start processing before full exploration completes
```

---

### Multi-Agent Workflow Examples

#### **Example 1: Domain Deep-Dive**
```
Main Agent Task: "Understand Lucene architecture"

Workflow:
  1. Main: Spawn sub-agent-1: "Explore lucene domain, give me map"
  2. Sub-1: Runs `kbase graph explore lucene/01-home.md --summary`
  3. Sub-1 returns: {clusters, hubs, reading_path}
  4. Main: "I see 3 clusters: indexing, search, analysis"
  5. Main: Spawn 3 sub-agents, one per cluster
     - Sub-2: "Fetch context for indexing cluster"
     - Sub-3: "Fetch context for search cluster"  
     - Sub-4: "Fetch context for analysis cluster"
  6. Main: Receives all 3 context bundles
  7. Main: Synthesizes understanding of entire domain
```

**Token Efficiency**: 
- Without sub-agents: 50K tokens (read everything)
- With sub-agents: 15K tokens (targeted exploration)

#### **Example 2: Cross-Domain Research**
```
Main Agent Task: "How do search engines handle high-cardinality fields?"

Workflow:
  1. Main: Spawn 3 sub-agents in parallel
     - Sub-1: Search lucene domain
     - Sub-2: Search elasticsearch domain
     - Sub-3: Search datafusion domain
  2. Each sub runs: `kbase search "high cardinality" --domain X`
  3. All return results in ~2 seconds (parallel)
  4. Main: Compares findings:
     - Lucene: Doc values, sparse encoding
     - Elasticsearch: Aggregations, cardinality estimation
     - DataFusion: Dictionary encoding
  5. Main: Synthesizes comparative answer
```

**Speed**: 3× faster than sequential search

#### **Example 3: Incremental Knowledge Building**
```
Main Agent Task: Working on code, occasionally needs context

Workflow:
  1. Main: Coding DataFusion feature
  2. Main: "Hmm, how does Lucene do this?"
  3. Spawn sub-agent: "Quick search: Lucene + <concept>"
  4. Sub returns: Snippet + note path
  5. Main: "Good enough" or "Need more detail"
  6. If more detail: "Fetch full note + context"
  7. Main: Continues coding with new knowledge
```

**Pattern**: Lightweight queries via sub-agents, full context only when needed

---

### Implementation Considerations

#### **1. Command Design for Sub-Agents**
All commands should support:
```bash
kbase <command> --format json --summary --max-tokens N
```

This trio enables:
- **JSON**: Structured output for programmatic parsing
- **Summary**: Condensed results (sub-agent → main agent communication)
- **Max tokens**: Budget control (prevent sub-agent from using all tokens)

#### **2. Exit Codes & Error Handling**
Sub-agents need clear success/failure signals:
```bash
kbase search "xyz" --domain lucene
# Exit 0: Found results
# Exit 1: No results found
# Exit 2: Error (domain doesn't exist, index missing, etc.)
```

Main agent can handle failures gracefully:
- Exit 1: Try different query/domain
- Exit 2: Check prerequisites (run `kbase index`?)

#### **3. Logging & Observability**
When debugging multi-agent workflows:
```bash
KBASE_LOG=debug kbase graph explore <note>
# Logs: Nodes visited, decisions made, timing
# Helps debug sub-agent behavior
```

#### **4. Timeouts**
Sub-agents shouldn't hang:
```bash
kbase graph explore <note> --timeout 30s
# Kills operation after 30 seconds
# Returns partial results or error
```

---

### Benefits of Multi-Agent Patterns

1. **Parallelization**: 3 domains explored in parallel = 3× speedup
2. **Specialization**: Sub-agents can have domain-specific prompts
3. **Token Efficiency**: Sub-agents return summaries, not full content
4. **Modularity**: Main agent orchestrates, sub-agents execute
5. **Fault Isolation**: Sub-agent failure doesn't crash main agent
6. **Exploration**: Sub-agents can explore autonomously, report findings

---

### Priority for Sub-Agent Support

**High Priority** (Implement in Phase 1-2):
- JSON output for all commands
- Summary/condensed modes
- Token budgeting (`--max-tokens`)

**Medium Priority** (Phase 2-3):
- Confidence scores in results
- Streaming output for long operations
- Better error codes and handling

**Low Priority** (Future):
- Advanced orchestration patterns
- Sub-agent result caching
- Cross-vault sub-agents

---

## Success Metrics

How do we know these features are valuable for agents?

1. **Context Quality**: Can agents answer complex questions using assembled context?
2. **Efficiency**: Tokens used per query (less is better with good context)
3. **Coverage**: % of agent queries that find relevant information
4. **Navigation**: Steps to find info (1 command vs 5+ manual steps)
5. **Discovery**: Can agents find information they didn't know to ask about?
6. **Parallelization**: Multi-agent speedup for cross-domain tasks (2-3× faster)

---

## Related Documents

- `plan/wikilinks-agentic.md` - Detailed wikilink feature brainstorm
- `plan/index.md` - Index architecture and storage
- `plan/ideas.md` - Additional innovation ideas
- `docs/tree-sitter.md` - Wikilink parsing implementation
- `docs/tags.md` - Current tag system
