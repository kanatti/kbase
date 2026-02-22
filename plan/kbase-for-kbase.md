# Using kbase to Develop kbase (Dogfooding)

**Status**: Planning  
**Date**: 2026-02-22  
**Goal**: Use kbase itself for kbase development - move from `plan/` markdown files to a proper kbase domain

---

## The Idea

Currently, kbase development planning is in `plan/*.md` files in the repo. But we're building a knowledge base tool - why not use it to organize kbase's own development?

**Dogfooding benefits:**
1. **Immediate UX feedback** - Feel pain points as a real user
2. **Feature validation** - See what's actually useful vs theoretical
3. **Test as you build** - New features get real-world testing instantly
4. **Cross-domain linking** - Connect kbase dev insights to Lucene/DataFusion knowledge
5. **Searchable dev knowledge** - `kbase search "context expansion"` beats grepping
6. **Natural organization** - Domains, tags, wikilinks instead of flat file structure

---

## Proposed Structure

Add a **`kbase/`** domain to your personal vault, following kbase conventions:

```
<your-vault>/
├── lucene/
├── elasticsearch/
├── datafusion/
├── rust/
│
├── kbase/                           # New: kbase development domain
│   ├── _description.md              # "Personal knowledge base CLI in Rust"
│   │
│   # Navigation & Status
│   ├── 01-home.md                   # Overview, current focus, quick links
│   ├── 02-logs.md                   # Development diary (chronological)
│   ├── 03-task-board.md             # Open tasks, priorities, blockers
│   ├── 04-archive.md                # Completed phases, historical decisions
│   │
│   # Architecture & Design
│   ├── architecture.md              # System design, components, data flow
│   ├── parser-design.md             # Tree-sitter, parsing strategy
│   ├── indexing-design.md           # Tantivy, link index, tags, storage
│   ├── cli-design.md                # Command structure, UX principles
│   │
│   # Feature Planning
│   ├── agentic-features.md          # Features for AI agents (from plan/agentic.md)
│   ├── wikilinks.md                 # Wikilink features (from plan/wikilinks-agentic.md)
│   ├── tags.md                      # Tag-based features
│   ├── search.md                    # Full-text + semantic search
│   ├── multi-agent-patterns.md      # Sub-agent delegation patterns
│   │
│   # Implementation Phases
│   ├── phase-1-foundation.md        # Link index, search, context expansion
│   ├── phase-2-intelligence.md      # Semantic search, smart assembly
│   ├── phase-3-advanced.md          # Concept extraction, summarization
│   │
│   # Technical Deep-Dives
│   ├── tree-sitter-integration.md   # How tree-sitter parsing works
│   ├── tantivy-integration.md       # Full-text search implementation
│   ├── link-resolution.md           # Wikilink resolution algorithm
│   ├── context-expansion.md         # Graph traversal, circular detection
│   │
│   # Development Guides
│   ├── testing-strategy.md          # Unit, integration, end-to-end tests
│   ├── rust-patterns.md             # Rust idioms used in kbase
│   ├── contributing.md              # How to contribute (if open-sourced)
```

### Domain Conventions

Following `docs/domains.md` patterns:

- **01-home.md**: High-level overview
  - Current phase
  - Focus areas
  - Quick links to key notes
  - Recent updates

- **02-logs.md**: Chronological development diary
  - Daily/weekly entries
  - Links to notes created/updated
  - Insights and discoveries
  - Problems encountered

- **03-task-board.md**: Active work tracking
  - Organized by phase/priority
  - Blockers and dependencies
  - Next steps

- **04-archive.md**: Historical record
  - Completed phases
  - Deprecated approaches
  - Lessons learned

---

## Example Notes with Wikilinks & Tags

### kbase/01-home.md
```markdown
# kbase Development

A personal knowledge base CLI for navigating markdown notes with wikilinks, tags, and full-text search.

## Current Focus

**Phase 1: Foundation** (#phase-1 #in-progress)

Building core infrastructure:
- [[kbase/indexing-design#Link Index]] - Backlinks and forward links
- [[kbase/search#Tantivy Integration]] - Full-text search
- [[kbase/context-expansion]] - Auto-assemble linked content

See [[kbase/phase-1-foundation]] for detailed plan.

## Key Design Decisions

- **Parser**: [[kbase/parser-design]] - Using tree-sitter for accurate parsing
- **Storage**: [[kbase/indexing-design]] - Tantivy + JSON indexes
- **Agent Focus**: [[kbase/agentic-features]] - Optimize for AI agent workflows

## Related Knowledge

This project builds on learnings from:
- [[lucene/search-flow]] - Search architecture patterns
- [[datafusion/query-execution]] - Graph traversal techniques
- [[rust/ownership]] - Rust patterns for CLI tools

## Recent Updates

- **2026-02-22**: Added [[kbase/wikilinks]] feature planning
- **2026-02-21**: Integrated [[kbase/tree-sitter-integration]]
- **2026-02-20**: Created [[kbase/agentic-features]] doc

#kbase #home #overview
```

### kbase/02-logs.md
```markdown
# Development Log

Chronological diary of kbase development.

---

## 2026-02-22

### Multi-Agent Patterns

Realized that [[kbase/agentic-features#Context Expansion]] would benefit from
sub-agent delegation. Added [[kbase/multi-agent-patterns]] to explore this.

Key insight: Sub-agents can explore graph autonomously and return condensed
summaries. This is way more efficient than main agent reading everything.

Related to [[datafusion/query-optimization#Parallel Execution]] - similar
parallelization benefits.

#dev-log #2026-02 #multi-agent

### Dogfooding Idea

Had the idea to use kbase for kbase development. Created [[kbase/kbase-for-kbase]]
to plan migration from `plan/` folder to proper vault domain.

This will let us test features as we build them. Immediate dogfooding feedback loop.

#dev-log #2026-02 #meta

---

## 2026-02-21

### Tree-sitter Integration

Finished implementing wikilink parsing with tree-sitter. See [[kbase/tree-sitter-integration]]
for technical details.

Parser now extracts:
- `[[note]]` - basic links
- `[[note|alias]]` - with display text
- `[[note#section]]` - with heading anchors
- `[[domain/note]]` - with paths

Next: Build [[kbase/indexing-design#Link Index]] to make these queryable.

#dev-log #2026-02 #parser #tree-sitter

---

## 2026-02-20

### Agentic Features Planning

Created comprehensive [[kbase/agentic-features]] doc exploring what AI agents
need from a knowledge base tool.

Key priorities:
1. Link index & backlinks
2. Context expansion
3. Full-text search
4. Smart context assembly

This will guide Phase 1 implementation.

#dev-log #2026-02 #planning #agentic
```

### kbase/wikilinks.md
```markdown
# Wikilink Features for Agents

How wikilinks enable powerful navigation and context assembly for AI agents.

See [[kbase/parser-design#Wikilinks]] for parsing implementation.

## Current Capabilities

We parse these wikilink formats (via [[kbase/tree-sitter-integration]]):

- `[[note]]` - Basic link
- `[[note|alias]]` - With display text
- `[[domain/note]]` - With path
- `[[note#section]]` - With heading anchor
- `[[domain/note#section|text]]` - All features combined

## Planned Features

### Phase 1: Link Index

Build bidirectional link graph. See [[kbase/phase-1-foundation#Link Index]].

```bash
kbase links <note>              # Outgoing links
kbase backlinks <note>          # Incoming links (who cites this?)
kbase links validate            # Check for broken links
```

**Implementation**: [[kbase/indexing-design#Link Index]]

### Phase 2: Context Expansion

Auto-assemble linked content. See [[kbase/context-expansion]].

```bash
kbase read <note> --expand-links --depth 2
kbase context <note>                          # Bidirectional
kbase bundle <note> --max-tokens 10000
```

**Agent Use Case**: LLM needs context about a topic, one command returns
note + all linked notes, assembled automatically.

Similar to how [[lucene/search-flow#Query Expansion]] works.

### Phase 3: Graph Queries

Advanced graph traversal. See [[kbase/agentic-features#Knowledge Graph Queries]].

```bash
kbase graph path <a> <b>           # Find connection
kbase graph neighborhood <note>     # Notes within N hops
kbase graph hubs                    # Highly connected notes
```

## Design Decisions

### Link Resolution

How do we resolve `[[note]]` to a file path?

See [[kbase/link-resolution]] for detailed algorithm.

Options considered:
1. Exact match only
2. Fuzzy search (risky, can be ambiguous)
3. Index-based lookup (chosen approach)

Decision: Build name → path index during `kbase index`, use for fast lookup.

### Circular References

What happens with `[[A]]` ↔ `[[B]]` during expansion?

See [[kbase/context-expansion#Circular Detection]].

Solution:
- Maintain visited set during traversal
- Depth limit (never expand beyond N hops)
- Both together for safety

Similar to [[datafusion/query-execution#Cycle Detection]] in recursive queries.

## Related

- [[kbase/agentic-features]] - Overall agent features
- [[kbase/parser-design]] - Parsing implementation
- [[kbase/indexing-design]] - Storage and indexes

#kbase #wikilinks #planning #deep-dive
```

---

## Benefits in Action

### 1. Cross-Domain Linking

Link kbase development notes to related concepts:

```markdown
# kbase/context-expansion.md

Graph traversal algorithm similar to [[lucene/search-flow#Query Traversal]].

Token budgeting follows [[datafusion/memory-management]] patterns.

Rust implementation uses [[rust/ownership#Borrowing]] to avoid clones.
```

Now `kbase backlinks rust/ownership.md` shows it's used in kbase development!

### 2. Tag-Based Discovery

```markdown
# All kbase notes use consistent tags:

#kbase              # Domain tag
#planning           # Planning docs
#implementation     # Implementation notes
#deep-dive          # Technical deep-dives
#phase-1            # Phase tags
#wip                # Work in progress
#done               # Completed
```

Then:

```bash
kbase notes --tag wip --domain kbase          # What's in progress?
kbase notes --tag phase-1                      # All phase 1 work
kbase notes --tag deep-dive --domain kbase    # Technical details
kbase notes --tag kbase --tag rust            # kbase + Rust intersect
```

### 3. Test Features Immediately

Building link index?
```bash
# As soon as it's implemented:
kbase index --only links
kbase backlinks kbase/wikilinks.md
kbase links kbase/agentic-features.md

# Does it work? Do the results make sense?
# Immediate feedback on UX and functionality
```

Building search?
```bash
kbase search "context expansion" --domain kbase
kbase search "wikilink resolution" --format snippets

# Are the results relevant? Is ranking good?
# Test with real content you care about
```

### 4. Development Queries

```bash
# What am I working on?
kbase notes --tag wip --domain kbase

# What's left in phase 1?
kbase notes --tag phase-1 --tag wip

# What technical deep-dives have I written?
kbase notes --tag deep-dive --domain kbase

# What mentions "tantivy"?
kbase search "tantivy" --domain kbase

# What links to the parser design?
kbase backlinks kbase/parser-design.md
```

### 5. Context for Agents

```bash
# Agent needs context about link index implementation
kbase context kbase/indexing-design --expand-links --depth 2

# Returns:
# - kbase/indexing-design.md (main note)
# - kbase/parser-design.md (linked)
# - kbase/link-resolution.md (linked)
# - kbase/wikilinks.md (links back)
# All assembled, ready for LLM
```

---

## Migration Plan

### Step 1: Set Up Domain

In your vault:

```bash
mkdir -p ~/path/to/vault/kbase
```

Create initial structure:
```bash
touch ~/path/to/vault/kbase/_description.md
touch ~/path/to/vault/kbase/01-home.md
touch ~/path/to/vault/kbase/02-logs.md
touch ~/path/to/vault/kbase/03-task-board.md
```

### Step 2: Migrate Content

Convert existing `plan/` files to kbase domain notes:

**From** → **To**:
- `plan/agentic.md` → `kbase/agentic-features.md`
- `plan/wikilinks-agentic.md` → `kbase/wikilinks.md`
- `plan/index.md` → `kbase/indexing-design.md`
- `plan/spec.md` → `kbase/architecture.md`
- `plan/implementation.md` → Split into phase notes
- `plan/ideas.md` → Extract to relevant feature notes

### Step 3: Add Wikilinks

Convert references to wikilinks:

**Before** (in plan/):
```markdown
See agentic.md for details on agent features.
```

**After** (in vault):
```markdown
See [[kbase/agentic-features]] for details on agent features.
```

### Step 4: Add Tags

Tag consistently:
```markdown
# Every note ends with relevant tags

#kbase #planning #phase-1 #wip
```

### Step 5: Build Index

```bash
kbase index
```

Now all kbase dev notes are searchable, linkable, discoverable!

### Step 6: Update Workflow

**Old workflow:**
1. Edit `plan/agentic.md`
2. Git commit
3. Maybe grep to find related notes

**New workflow:**
1. Edit `kbase/agentic-features.md` in vault
2. Git commit vault (if versioned)
3. `kbase backlinks kbase/agentic-features.md` - see what's related
4. `kbase context kbase/agentic-features.md` - get full context

---

## Workflow Examples

### Daily Development Routine

**Morning: Review status**
```bash
kbase read kbase/01-home.md                  # What's current?
kbase notes --tag wip --domain kbase         # What's in progress?
kbase read kbase/03-task-board.md            # What's on deck?
```

**During: Work on feature**
```bash
# Working on link index
kbase read kbase/phase-1-foundation.md --outline    # Refresh on plan
kbase context kbase/indexing-design.md              # Get full context

# While coding, update notes:
vim ~/vault/kbase/02-logs.md                        # Add entry
vim ~/vault/kbase/link-resolution.md                # Document decisions
```

**End of day: Log progress**
```bash
# Add to log
vim ~/vault/kbase/02-logs.md

# Content:
## 2026-02-22
Implemented [[kbase/link-resolution]] algorithm.
Links to [[kbase/parser-design#Wikilinks]].
Next: Build [[kbase/indexing-design#Storage]].
#dev-log #2026-02 #link-index

# Rebuild index
kbase index
```

### Planning New Feature

```bash
# Research similar features
kbase search "graph traversal"                      # Existing knowledge
kbase notes --tag deep-dive                         # Related deep-dives

# Create feature note
vim ~/vault/kbase/semantic-search.md

# Link to related concepts
# [[kbase/search]] - existing search design
# [[lucene/bkd-trees]] - spatial indexing inspiration
# [[kbase/phase-2-intelligence]] - where it fits

# Tag appropriately
#kbase #planning #phase-2 #semantic-search #wip
```

### Context for AI Agent

```bash
# Agent asks: "How should we implement context expansion?"

# Get comprehensive context:
kbase bundle kbase/context-expansion.md --expand-links --depth 2

# Returns JSON with:
# - kbase/context-expansion.md (main note)
# - kbase/link-resolution.md (how to resolve links)
# - kbase/indexing-design.md (data structures)
# - kbase/agentic-features.md (requirements)
# - lucene/search-flow.md (similar patterns)

# Agent has everything needed to help implement
```

---

## Validation Metrics

How do we know dogfooding is working?

### Feature Quality Indicators

1. **Immediate pain points discovered**
   - "This command is annoying to type"
   - "I expected this output format"
   - "Why can't I filter by X?"

2. **Features that get used vs ignored**
   - Do you actually use `kbase tags` daily?
   - Is `--format json` essential or theoretical?
   - Which commands are in your shell history?

3. **Missing features become obvious**
   - "Wish I could..."
   - "Why can't I..."
   - "Would be nice if..."

### Development Workflow Indicators

1. **Faster context switching**
   - Time to get up to speed on feature: Before vs After
   - Steps to find related notes: Before vs After

2. **Better knowledge retention**
   - Can you find that design decision from 2 weeks ago?
   - Do wikilinks help you rediscover connections?

3. **Cross-domain insights**
   - Do kbase dev notes link to other domains?
   - Do you discover patterns across projects?

---

## Risks & Mitigations

### Risk: Circular Dependency
"Can't use kbase until kbase is built"

**Mitigation**: Start with minimal features:
- Use basic `kbase notes`, `kbase read` (already works)
- Don't depend on unimplemented features
- Incrementally add features as they're built

### Risk: Premature Optimization
"Spending too much time on kbase instead of building it"

**Mitigation**: Time-box organization:
- 15 min/day max on note organization
- Focus on notes that support current work
- Let structure emerge naturally

### Risk: Over-Engineering
"Making kbase too complex because you're using it"

**Mitigation**: Remember the goal:
- Agents are primary users, not you
- Your workflow informs design but doesn't dictate it
- Stay focused on agent use cases from [[kbase/agentic-features]]

---

## Next Steps

1. **Set up domain structure** (30 min)
   - Create `kbase/` directory in vault
   - Add `01-home.md`, `02-logs.md`, etc.
   - Write `_description.md`

2. **Migrate 1-2 key docs** (1 hour)
   - Start with `plan/agentic.md` → `kbase/agentic-features.md`
   - Add wikilinks and tags
   - Test with `kbase read`, `kbase notes`

3. **Use for 1 week** (ongoing)
   - Daily log in `02-logs.md`
   - Update task board
   - Create notes as needed
   - Capture pain points

4. **Evaluate** (after 1 week)
   - Is it useful?
   - What's working?
   - What's missing?
   - Adjust approach

---

## Success Criteria

After 2 weeks of dogfooding, we should be able to:

✅ Find any design decision in < 30 seconds  
✅ Get context for any feature with one command  
✅ Discover related notes through backlinks  
✅ Track work in progress via tags  
✅ See how kbase concepts relate to other domains  
✅ Have discovered 3+ UX improvements from real usage  

If these are true, dogfooding is working and kbase is on the right track.

---

## Related Documents

- [[kbase/agentic-features]] - What agents need from kbase
- [[kbase/architecture]] - System design
- [[kbase/phase-1-foundation]] - Current implementation phase
- `plan/` folder - Legacy planning docs (will migrate from)
- `docs/domains.md` - Domain conventions (repo docs)
