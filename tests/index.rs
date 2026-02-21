mod common;

use predicates::prelude::*;
use std::fs;

#[test]
fn test_index_command_builds_tag_index() {
    let vault = common::setup_vault();

    // Run kbase index command using the fixture vault
    let mut cmd = common::kbase(&vault);
    cmd.arg("index");

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Building tag index..."))
        .stdout(predicate::str::contains("Built tag index:"));

    // Verify tags.json was created in the centralized location
    let tags_json_path = vault.path().join(".kbase/test-vault/tags.json");
    assert!(
        tags_json_path.exists(),
        "tags.json should be created at {}",
        tags_json_path.display()
    );

    // Verify tags.json contents
    let tags_content = fs::read_to_string(&tags_json_path).unwrap();
    let tags: serde_json::Value = serde_json::from_str(&tags_content).unwrap();

    // Check expected tags from our fixture files
    assert!(tags.get("deep-dive").is_some());
    assert!(tags.get("lucene").is_some());
    assert!(tags.get("elasticsearch").is_some());
    assert!(tags.get("performance").is_some());

    // Check tag-to-paths mappings
    let deep_dive_notes = tags["deep-dive"].as_array().unwrap();
    assert!(deep_dive_notes.len() >= 2); // Should appear in multiple notes
}

#[test]
fn test_index_command_with_only_tags_filter() {
    let vault = common::setup_vault();

    // Run kbase index --only tags
    let mut cmd = common::kbase(&vault);
    cmd.args(["index", "--only", "tags"]);

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Building tag index..."))
        .stdout(predicate::str::contains("Built tag index:"));
}

#[test]
fn test_index_command_skips_code_block_tags() {
    let vault = common::setup_vault();

    let mut cmd = common::kbase(&vault);
    cmd.arg("index");

    cmd.assert().success();

    // Check tags.json in centralized location
    let tags_json_path = vault.path().join(".kbase/test-vault/tags.json");
    let tags_content = fs::read_to_string(&tags_json_path).unwrap();
    let tags: serde_json::Value = serde_json::from_str(&tags_content).unwrap();

    // Should not have fake tags from code blocks
    assert!(tags.get("fake-tag").is_none());
    assert!(tags.get("also-fake").is_none());
}

#[test]
fn test_index_command_invalid_only_option() {
    let vault = common::setup_vault();

    let mut cmd = common::kbase(&vault);
    cmd.args(["index", "--only", "invalid"]);

    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("invalid value 'invalid'"));
}
