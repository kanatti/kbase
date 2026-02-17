# Search Flow Deep Dive

How TermQuery flows through IndexSearcher, Weight, Scorer and into TopDocs.

## Phase 1: IndexSearcher.search()

Entry point for all searches in Lucene.

### Step 1: createWeight()

Weight wraps the query for reuse across segments.

### Step 2: BulkScorer

Scores documents in bulk for a segment.

## Phase 2: Scoring

Final BM25 scoring and TopDocs collection.
