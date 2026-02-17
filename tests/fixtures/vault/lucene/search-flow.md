# Search Flow Deep Dive

How TermQuery flows through IndexSearcher, Weight, Scorer and into TopDocs.

## Phase 1: IndexSearcher.search()

Entry point for all searches in Lucene.
