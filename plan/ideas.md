# Agent-Focused Innovation Ideas for kbase

Based on the docs and current implementation, here are innovative ideas focused on how **AI agents** will use kbase for finding details they need:

## 1. Semantic Context Retrieval (Most Valuable)
```bash
kbase context --for "implementing BKD tree indexing in datafusion"
```
**What it does:** Uses embeddings + keyword hybrid search to pull the TOP relevant snippets across all domains. Returns:
- Relevant note excerpts (with line numbers)
- Domain context (what domain each came from)
- Confidence scores
- Suggested follow-up notes

**Agent use case:** Pi is working on DataFusion code, needs to know "how does Lucene handle BKD trees?" - one command pulls all relevant context without the agent needing to know which domains or tags to search.

**Implementation:**
- Build embedding index during `kbase index` (using local model like nomic-embed)
- Hybrid search: semantic similarity + BM25
- Return ranked snippets, not full notes
- Cache embeddings per-paragraph

## 2. Wikilink Suggestion Engine
```bash
kbase suggest-links lucene/new-codec-analysis.md
```
**What it does:** Analyzes a note's content and suggests wikilinks to existing notes based on:
- Named entities (class names, concepts)
- Semantic similarity to existing notes
- Frequent co-occurrence patterns

**Agent use case:** When Pi writes a new note or appends to existing ones, it can auto-suggest relevant internal links to maintain graph connectivity.

**Output:**
```
Suggested links for lucene/new-codec-analysis.md:

Line 12: "block tree terms dictionary" → [[lucene/codecs-deep-dive#Block Trees]]
Line 45: "posting lists" → [[lucene/postings-format]]
Line 78: "doc values implementation" → [[lucene/doc-values]]
```

## 3. Concept Extraction & Indexing
```bash
kbase concepts
kbase concepts --domain lucene
kbase concept "BKD tree" --notes  # which notes discuss this concept?
kbase concept "BKD tree" --definition  # extract the definition
```

**What it does:** NER-style extraction identifies key technical concepts, indexes them with:
- First definition found
- All mentions across notes
- Related concepts (co-occurrence)

**Agent use case:** Agent asks "what is a posting list?" → kbase returns the definition + all notes that discuss it, without needing exact string match.

## 4. Dependency Graph (Knowledge Prerequisites)
```bash
kbase prerequisites lucene/search-flow-deep-dive.md
```

**What it does:** Analyzes note content + links to build a "you should read these first" graph. Uses:
- Wikilink structure
- Complexity heuristics (intro vs deep-dive tags)
- Folder conventions (01-home before detailed notes)

**Agent output:**
```
To understand lucene/search-flow-deep-dive.md, read these first:

Required:
  1. lucene/01-home.md (domain overview)
  2. lucene/indexing-basics.md (referenced concepts)
  
Recommended:
  3. lucene/codecs-deep-dive.md (related deep-dive)
  4. datafusion/query-execution.md (similar patterns)
```

**Agent use case:** When researching unfamiliar territory, agent asks for reading order instead of randomly navigating.

## 5. Note Summarization with Key Facts
```bash
kbase summarize lucene/search-flow-deep-dive.md
kbase summarize lucene/search-flow-deep-dive.md --extract facts
kbase summarize lucene/search-flow-deep-dive.md --extract code-snippets
```

**What it does:** LLM-powered summarization that:
- Extracts key factual statements
- Identifies code examples
- Lists main concepts covered
- Returns structured output (JSON for agents)

**Agent use case:** Instead of reading 2000-line deep-dive notes, agent gets condensed facts: "This note covers: 1) TermQuery flow, 2) Weight creation, 3) Scorer implementation..."

## 6. Smart Multi-Domain Research Sessions (Enhanced version of planned feature)
```bash
kbase research lucene datafusion --question "How do both systems handle columnar compression?"
```

**What it does:** Not just listing notes, but:
- Identifies relevant notes in BOTH domains about the topic
- Extracts comparable sections
- Highlights similarities/differences
- Returns comparative summary

**Agent output:**
```
Research: lucene vs datafusion on "columnar compression"

Lucene approach:
  From lucene/doc-values.md:
    - Uses sparse encoding for low-cardinality fields
    - Implements run-length encoding for repeated values
    
DataFusion approach:
  From datafusion/parquet-integration.md:
    - Delegates to Parquet's columnar compression
    - Supports dictionary, RLE, bit-packing

Key difference: Lucene implements compression; DataFusion uses Parquet's.
Related notes: [arrow/columnar-format.md, elasticsearch/esql-analysis.md]
```

## 7. Diff/Compare Notes
```bash
kbase diff lucene/codecs-v90.md lucene/codecs-v94.md
kbase compare elasticsearch/esql-query-flow.md datafusion/query-execution.md --concepts
```

**What it does:** 
- Structural diff of headings/sections
- Concept-level comparison (what's new, what changed)
- Identifies unique vs shared information

**Agent use case:** Understanding evolution of concepts or comparing approaches across domains.

## 8. Missing Knowledge Detector
```bash
kbase gaps --domain lucene
kbase gaps --concept "search algorithms"
```

**What it does:** Identifies knowledge gaps by:
- Analyzing unresolved wikilinks → concepts you planned to document
- Finding referenced concepts (in text) without dedicated notes
- Detecting isolated notes (low connectivity)
- Comparing domain coverage (why does domain X have 5× more notes than Y?)

**Agent output:**
```
Knowledge gaps in lucene:

Unresolved links (5):
  - esql-optimizer (referenced in 3 notes, no note exists)
  - memory-management (mentioned 7 times, no dedicated note)
  
Mentioned but not documented:
  - "segment merging" (7 occurrences, no [[wikilink]])
  - "index warming" (3 occurrences, no [[wikilink]])
  
Suggested: Create notes for high-frequency undocumented concepts.
```

## 9. Agent Memory/Cache Layer
```bash
kbase cache --save "current-context" --notes lucene/search-flow.md datafusion/execution.md
kbase cache --restore "current-context"
kbase cache --list
```

**What it does:** Let agents save "research sessions" - collections of relevant notes for a specific task. Restored context includes:
- Note paths
- Key excerpts
- Related concepts
- Timestamp

**Agent use case:** Pi works on multiple long-running tasks. Each session needs different context. Cache makes context switches fast.

## 10. Auto-Generated Index Notes
```bash
kbase auto-index --domain lucene --output lucene/00-auto-index.md
kbase auto-index --tag deep-dive --output meta/deep-dives-index.md
```

**What it does:** Generate comprehensive index notes with:
- All notes in domain/tag with summaries
- Concept map (what's covered)
- Link graph visualization (ASCII)
- Reading path suggestions

**Agent use case:** Auto-maintain overview/navigation notes without manual curation.

## Priority Ranking for Agent Workflows

### Must-Have (Tier 1)
1. **Semantic Context Retrieval** - Agents need relevant context fast, across domains
2. **Missing Knowledge Detector** - Helps agents know what info is lacking
3. **Concept Extraction** - Makes knowledge queryable by concept, not just keywords

### High-Value (Tier 2)
4. **Smart Multi-Domain Research** - Agents work across domains constantly
5. **Wikilink Suggestion** - Improves knowledge graph connectivity automatically
6. **Prerequisites Graph** - Agents need reading order for unfamiliar topics

### Nice-to-Have (Tier 3)
7. **Note Summarization** - Reduces token usage when pulling context
8. **Agent Memory Cache** - Optimizes repeated access patterns
9. **Auto-Generated Indexes** - Reduces manual maintenance
10. **Diff/Compare** - Specialized use cases

## Technical Considerations

**Embedding Model:** Use `nomic-embed-text` (local, fast, 8192 context)
**Hybrid Search:** Combine semantic + BM25 for best recall
**Storage:** Add `embeddings.db` (SQLite with vector extension) or use FAISS
**Incremental Updates:** Only re-embed changed paragraphs during `kbase index`

These features transform kbase from a "search tool" into an **active knowledge assistant** that helps agents navigate, synthesize, and extend your knowledge base.
