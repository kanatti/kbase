use crate::vault::{Note, Vault};
use anyhow::{Result, bail};
use serde::Serialize;
use std::path::PathBuf;

/// A note prepared for full-text indexing
#[derive(Debug, Clone)]
pub struct IndexedNote {
    path: PathBuf,
    domain: String,        // Empty string for root-level notes
    title: String,
    content: String,
}

impl IndexedNote {
    /// Create an IndexedNote from a vault Note
    pub fn from_vault_note(note: &Note, vault: &Vault) -> Result<Self> {
        let path = note.path.clone();
        let domain = extract_domain(&path);
        let title = note.title.clone();
        
        // Read full content
        let content = vault.read_note(&note.path.to_string_lossy())?;
        
        Ok(IndexedNote {
            path,
            domain,
            title,
            content,
        })
    }
    
    pub fn path(&self) -> &PathBuf { &self.path }
    pub fn domain(&self) -> &str { &self.domain }
    pub fn title(&self) -> &str { &self.title }
    pub fn content(&self) -> &str { &self.content }
    
    /// Check if this note belongs to a specific domain
    pub fn is_in_domain(&self, domain: &str) -> bool {
        self.domain == domain
    }
}

/// Extract domain from vault-relative path
/// Returns empty string for root-level notes
pub fn extract_domain(path: &PathBuf) -> String {
    // Only return domain if path has at least one directory separator
    if path.components().count() < 2 {
        return String::new();
    }
    
    path.components()
        .next()
        .and_then(|c| c.as_os_str().to_str())
        .unwrap_or("")
        .to_string()
}

/// A search query with filters and options
#[derive(Debug, Clone)]
pub struct SearchQuery {
    term: String,
    domain_filter: Option<String>,
    limit: usize,
}

impl SearchQuery {
    /// Create a new search query
    pub fn new(term: impl Into<String>) -> Result<Self> {
        let term = term.into();
        
        // Validate term is not empty/whitespace
        if term.trim().is_empty() {
            bail!("Search term cannot be empty");
        }
        
        Ok(SearchQuery {
            term,
            domain_filter: None,
            limit: 50, // default
        })
    }
    
    /// Builder: Add domain filter
    pub fn with_domain(mut self, domain: impl Into<String>) -> Self {
        self.domain_filter = Some(domain.into());
        self
    }
    
    /// Builder: Set result limit
    pub fn with_limit(mut self, limit: usize) -> Result<Self> {
        if limit == 0 {
            bail!("Result limit must be > 0");
        }
        self.limit = limit;
        Ok(self)
    }
    
    pub fn term(&self) -> &str { &self.term }
    pub fn domain_filter(&self) -> Option<&str> { self.domain_filter.as_deref() }
    pub fn limit(&self) -> usize { self.limit }
    
    /// Check if query should filter by domain
    pub fn has_domain_filter(&self) -> bool {
        self.domain_filter.is_some()
    }
}

/// Search relevance score (BM25)
/// 
/// Invariants:
/// - Score must be >= 0.0
/// - Higher score = more relevant
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize)]
pub struct Relevance(f32);

impl Relevance {
    /// Create a new relevance score
    pub fn new(score: f32) -> Result<Self> {
        if score < 0.0 {
            bail!("Relevance score cannot be negative: {}", score);
        }
        if !score.is_finite() {
            bail!("Relevance score must be finite: {}", score);
        }
        Ok(Relevance(score))
    }
    
    /// Get the raw score
    pub fn as_f32(&self) -> f32 { self.0 }
    
    /// Format score for display (2 decimal places)
    pub fn format(&self) -> String {
        format!("{:.2}", self.0)
    }
}

impl std::fmt::Display for Relevance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}", self.0)
    }
}

/// A single search result
#[derive(Debug, Clone, Serialize)]
pub struct SearchResult {
    pub path: String,
    pub title: String,
    pub domain: String,
    pub relevance: Relevance,
}

impl SearchResult {
    pub fn new(path: String, title: String, domain: String, relevance: Relevance) -> Self {
        SearchResult { path, title, domain, relevance }
    }
    
    /// Check if result is from a specific domain
    pub fn is_from_domain(&self, domain: &str) -> bool {
        self.domain == domain
    }
    
    /// Check if relevance meets a threshold
    pub fn meets_threshold(&self, threshold: f32) -> bool {
        self.relevance.as_f32() >= threshold
    }
}

/// Collection of search results with metadata
#[derive(Debug, Clone, Serialize)]
pub struct SearchResults {
    results: Vec<SearchResult>,
    query: String,
    total_count: usize,
}

impl SearchResults {
    pub fn new(results: Vec<SearchResult>, query: String) -> Self {
        let total_count = results.len();
        SearchResults { results, query, total_count }
    }
    
    pub fn results(&self) -> &[SearchResult] { &self.results }
    pub fn query(&self) -> &str { &self.query }
    pub fn count(&self) -> usize { self.total_count }
    pub fn is_empty(&self) -> bool { self.results.is_empty() }
    
    /// Filter results by minimum relevance threshold
    pub fn filter_by_relevance(mut self, min_score: f32) -> Self {
        self.results.retain(|r| r.meets_threshold(min_score));
        self.total_count = self.results.len();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relevance_validation() {
        assert!(Relevance::new(-1.0).is_err());
        assert!(Relevance::new(f32::NAN).is_err());
        assert!(Relevance::new(f32::INFINITY).is_err());
        assert!(Relevance::new(0.0).is_ok());
        assert!(Relevance::new(5.5).is_ok());
    }

    #[test]
    fn test_relevance_display() {
        let r = Relevance::new(5.678).unwrap();
        assert_eq!(r.format(), "5.68");
        assert_eq!(r.to_string(), "5.68");
    }

    #[test]
    fn test_search_query_validation() {
        assert!(SearchQuery::new("").is_err());
        assert!(SearchQuery::new("   ").is_err());
        assert!(SearchQuery::new("valid").is_ok());
    }

    #[test]
    fn test_search_query_builder() {
        let query = SearchQuery::new("rust")
            .unwrap()
            .with_domain("lucene")
            .with_limit(10)
            .unwrap();
        
        assert_eq!(query.term(), "rust");
        assert_eq!(query.domain_filter(), Some("lucene"));
        assert_eq!(query.limit(), 10);
        assert!(query.has_domain_filter());
    }

    #[test]
    fn test_search_query_limit_validation() {
        let query1 = SearchQuery::new("test").unwrap();
        assert!(query1.with_limit(0).is_err());
        
        let query2 = SearchQuery::new("test").unwrap();
        assert!(query2.with_limit(1).is_ok());
    }

    #[test]
    fn test_extract_domain() {
        assert_eq!(extract_domain(&PathBuf::from("lucene/search.md")), "lucene");
        assert_eq!(extract_domain(&PathBuf::from("rust/ownership/basics.md")), "rust");
        assert_eq!(extract_domain(&PathBuf::from("01-home.md")), "");
    }

    #[test]
    fn test_result_filtering() {
        let results = vec![
            SearchResult::new("a.md".into(), "A".into(), "".into(), Relevance::new(10.0).unwrap()),
            SearchResult::new("b.md".into(), "B".into(), "".into(), Relevance::new(5.0).unwrap()),
            SearchResult::new("c.md".into(), "C".into(), "".into(), Relevance::new(2.0).unwrap()),
        ];
        
        let search_results = SearchResults::new(results, "test".into());
        let filtered = search_results.filter_by_relevance(5.0);
        
        assert_eq!(filtered.count(), 2);
        assert!(filtered.results().iter().all(|r| r.relevance.as_f32() >= 5.0));
    }
}
