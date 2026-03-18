//! Full-text search for markdown notes using Tantivy.
//!
//! Use [`SearchIndexer`] to build indexes and [`SearchService`] to execute queries.

// Re-exports
pub use indexer::SearchIndexer;
pub use searcher::SearchService;
pub use repository::SearchRepository;
pub use domain::{SearchQuery, SearchResult};

// Submodules
mod domain;
mod schema;
mod indexer;
mod searcher;
mod repository;
