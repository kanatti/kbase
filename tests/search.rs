mod common;

use anyhow::Result;
use kbase::search::{SearchIndexer, SearchService, SearchRepository, SearchQuery};
use kbase::vault::Vault;
use serial_test::serial;
use tempfile::TempDir;

#[test]
#[serial]
fn test_build_search_index() -> Result<()> {
    let tmp = common::setup_vault();
    
    // Create temp directory for index storage
    let temp_index_dir = TempDir::new()?;
    
    // Set KBASE_HOME to this test's temp directory
    unsafe {
        std::env::set_var("KBASE_HOME", temp_index_dir.path());
    }
    
    let vault = Vault::open(
        tmp.path().to_path_buf(),
        "test-vault".into(),
    )?;
    
    let indexer = SearchIndexer::new();
    let stats = indexer.build_from_vault(&vault)?;
    assert!(stats.indexed_count() > 0, "Should index at least one document");
    
    // Verify index directory created
    let index_path = vault.index_dir()?.join("search.tantivy");
    assert!(index_path.exists(), "Index directory should exist");
    
    Ok(())
}

#[test]
#[serial]
fn test_search_basic_query() -> Result<()> {
    let tmp = common::setup_vault();
    
    // Create temp directory for index storage
    let temp_index_dir = TempDir::new()?;
    
    // Set KBASE_HOME to this test's temp directory
    unsafe {
        std::env::set_var("KBASE_HOME", temp_index_dir.path());
    }
    
    let vault = Vault::open(
        tmp.path().to_path_buf(),
        "test-vault".into(),
    )?;
    
    // Build index
    let indexer = SearchIndexer::new();
    let stats = indexer.build_from_vault(&vault)?;
    
    // Search for a term that should exist in test fixtures
    let query = SearchQuery::new("search")?.with_limit(10)?;
    
    let index_path = vault.index_dir()?.join("search.tantivy");
    let repository = SearchRepository::new(index_path);
    let service = SearchService::new(repository);
    
    let results = service.search(&query)?;
    
    assert!(!results.is_empty(), "Should find results for 'search'");
    assert!(results.results()[0].relevance.as_f32() > 0.0, "Results should have scores");
    
    Ok(())
}

#[test]
#[serial]
fn test_search_with_domain_filter() -> Result<()> {
    let tmp = common::setup_vault();
    
    // Create temp directory for index storage
    let temp_index_dir = TempDir::new()?;
    
    // Set KBASE_HOME to this test's temp directory
    unsafe {
        std::env::set_var("KBASE_HOME", temp_index_dir.path());
    }
    
    let vault = Vault::open(
        tmp.path().to_path_buf(),
        "test-vault".into(),
    )?;
    
    let indexer = SearchIndexer::new();
    let stats = indexer.build_from_vault(&vault)?;
    
    // Search within specific domain
    let query = SearchQuery::new("lucene")?
        .with_domain("lucene")
        .with_limit(10)?;
    
    let index_path = vault.index_dir()?.join("search.tantivy");
    let repository = SearchRepository::new(index_path);
    let service = SearchService::new(repository);
    
    let results = service.search(&query)?;
    
    // All results should be from lucene domain
    for result in results.results() {
        assert!(
            result.domain == "lucene" || result.path.starts_with("lucene/"),
            "Result should be from lucene domain: {}",
            result.path
        );
    }
    
    Ok(())
}

#[test]
#[serial]
fn test_search_empty_query() -> Result<()> {
    let tmp = common::setup_vault();
    
    // Create temp directory for index storage
    let temp_index_dir = TempDir::new()?;
    
    // Set KBASE_HOME to this test's temp directory
    unsafe {
        std::env::set_var("KBASE_HOME", temp_index_dir.path());
    }
    
    let vault = Vault::open(
        tmp.path().to_path_buf(),
        "test-vault".into(),
    )?;
    
    let indexer = SearchIndexer::new();
    let stats = indexer.build_from_vault(&vault)?;
    
    // Empty query should be rejected with an error
    let result = SearchQuery::new("");
    assert!(result.is_err(), "Empty query should be rejected");
    
    let err_msg = format!("{}", result.unwrap_err());
    assert!(err_msg.contains("cannot be empty"), 
            "Error should mention empty term: {}", err_msg);
    
    Ok(())
}

#[test]
#[serial]
fn test_search_missing_index() {
    let tmp = common::setup_vault();
    
    // Create temp directory for index storage
    let temp_index_dir = TempDir::new().unwrap();
    
    // Set KBASE_HOME to this test's temp directory
    unsafe {
        std::env::set_var("KBASE_HOME", temp_index_dir.path());
    }
    
    let vault = Vault::open(
        tmp.path().to_path_buf(),
        "test-vault".into(),
    ).unwrap();
    
    let index_path = vault.index_dir().unwrap().join("nonexistent-index");
    let repository = SearchRepository::new(index_path);
    let service = SearchService::new(repository);
    
    let query = SearchQuery::new("test").unwrap();
    
    // Should return helpful error
    let result = service.search(&query);
    assert!(result.is_err(), "Should error on missing index");
    
    let err_msg = format!("{}", result.unwrap_err());
    assert!(err_msg.contains("not found") || err_msg.contains("kbase index"), 
            "Error should mention missing index: {}", err_msg);
}
