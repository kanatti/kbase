mod common;

use common::{kbase, kbase_persist, setup_vault};
use predicates::str::contains;

#[test]
fn repo_list_empty() {
    let tmp = setup_vault();
    
    // Remove _repos directory
    std::fs::remove_dir_all(tmp.path().join("_repos")).unwrap();
    
    kbase(&tmp)
        .args(["repo", "list"])
        .assert()
        .success()
        .stdout(contains("No repositories found"));
}

#[test]
fn repo_list() {
    let tmp = setup_vault();
    
    kbase(&tmp)
        .args(["repo", "list"])
        .assert()
        .success()
        .stdout(contains("datafusion"))
        .stdout(contains("tantivy"))
        .stdout(contains("⚠ no")); // No paths configured
}

#[test]
fn repo_list_json() {
    let tmp = setup_vault();
    
    kbase(&tmp)
        .args(["repo", "list", "--json"])
        .assert()
        .success()
        .stdout(contains("\"name\": \"datafusion\""))
        .stdout(contains("\"configured\": false"));
}

#[test]
fn repo_describe() {
    let tmp = setup_vault();
    
    kbase(&tmp)
        .args(["repo", "describe", "--name", "datafusion"])
        .assert()
        .success()
        .stdout(contains("Repository: datafusion"))
        .stdout(contains("Path: ⚠ not set"))
        .stdout(contains("Apache DataFusion"))
        .stdout(contains("Query engine in Rust"));
}

#[test]
fn repo_describe_json() {
    let tmp = setup_vault();
    
    kbase(&tmp)
        .args(["repo", "describe", "--name", "tantivy", "--json"])
        .assert()
        .success()
        .stdout(contains("\"name\": \"tantivy\""))
        .stdout(contains("\"configured\": false"))
        .stdout(contains("Full-text search"));
}

#[test]
fn repo_describe_not_found() {
    let tmp = setup_vault();
    
    kbase(&tmp)
        .args(["repo", "describe", "--name", "nonexistent"])
        .assert()
        .failure()
        .stderr(contains("not found"));
}

#[test]
fn repo_configure() {
    let tmp = setup_vault();
    
    // Configure a path (use tmp path itself as it exists)
    kbase_persist(&tmp)
        .args(["repo", "configure", "--name", "datafusion", "--path", tmp.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(contains("✓ Configured datafusion"));
    
    // Verify it was saved
    kbase_persist(&tmp)
        .args(["repo", "describe", "--name", "datafusion"])
        .assert()
        .success()
        .stdout(contains("Path:"))
        .stdout(contains(tmp.path().to_str().unwrap()));
}

#[test]
fn repo_configure_invalid_path() {
    let tmp = setup_vault();
    
    kbase(&tmp)
        .args(["repo", "configure", "--name", "datafusion", "--path", "/nonexistent/path"])
        .assert()
        .failure()
        .stderr(contains("does not exist"));
}

#[test]
fn repo_configure_nonexistent_repo() {
    let tmp = setup_vault();
    
    kbase(&tmp)
        .args(["repo", "configure", "--name", "nonexistent", "--path", tmp.path().to_str().unwrap()])
        .assert()
        .failure()
        .stderr(contains("not found"));
}

#[test]
fn repo_list_shows_configured() {
    let tmp = setup_vault();
    
    // Configure one repo
    kbase_persist(&tmp)
        .args(["repo", "configure", "--name", "datafusion", "--path", tmp.path().to_str().unwrap()])
        .assert()
        .success();
    
    // List should show mixed status
    let output = kbase_persist(&tmp)
        .args(["repo", "list"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();
    
    let stdout = String::from_utf8(output).unwrap();
    
    // Check datafusion shows "yes" and tantivy shows "no"
    assert!(stdout.contains("datafusion") && stdout.contains("✓ yes"));
    assert!(stdout.contains("tantivy") && stdout.contains("⚠ no"));
}
