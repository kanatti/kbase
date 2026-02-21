mod common;

use common::{kbase, setup_vault};
use predicates::str::contains;

#[test]
fn notes_lists_all_notes_without_domain() {
    let tmp = setup_vault();

    kbase(&tmp)
        .arg("notes")
        .assert()
        .success()
        .stdout(contains("elasticsearch"))
        .stdout(contains("lucene"))
        .stdout(contains("rust"));
}

#[test]
fn notes_domain_filter_lists_notes_in_domain() {
    let tmp = setup_vault();

    kbase(&tmp)
        .args(["notes", "--domain", "lucene"])
        .assert()
        .success()
        .stdout(contains("01-home.md"))
        .stdout(contains("search-flow.md"))
        .stdout(contains("codecs.md"));
}

#[test]
fn notes_domain_filter_shows_titles() {
    let tmp = setup_vault();

    kbase(&tmp)
        .args(["notes", "--domain", "lucene"])
        .assert()
        .success()
        .stdout(contains("Lucene"))
        .stdout(contains("Search Flow Deep Dive"));
}

#[test]
fn notes_domain_filter_falls_back_to_stem_when_no_heading() {
    let tmp = setup_vault();

    kbase(&tmp)
        .args(["notes", "--domain", "lucene"])
        .assert()
        .success()
        .stdout(contains("codecs"));
}

#[test]
fn notes_files_flag_hides_titles() {
    let tmp = setup_vault();

    let out = kbase(&tmp)
        .args(["notes", "--domain", "lucene", "--files"])
        .assert()
        .success();
    let stdout = String::from_utf8(out.get_output().stdout.clone()).unwrap();

    assert!(stdout.contains("search-flow.md"));
    assert!(
        !stdout.contains("Search Flow Deep Dive"),
        "title should be hidden with --files"
    );
}

#[test]
fn notes_unknown_domain_gives_error() {
    let tmp = setup_vault();

    kbase(&tmp)
        .args(["notes", "--domain", "nonexistent"])
        .assert()
        .failure()
        .stderr(contains("Error"));
}

#[test]
fn notes_term_not_yet_implemented() {
    let tmp = setup_vault();

    kbase(&tmp)
        .args(["notes", "--term", "search"])
        .assert()
        .failure()
        .stderr(contains("not yet implemented"));
}

// ============================================================================
// Tag functionality tests
// ============================================================================

#[test]
fn notes_tag_requires_index() {
    let tmp = setup_vault();

    // Without building index first, --tag should fail
    kbase(&tmp)
        .args(["notes", "--tag", "rust"])
        .assert()
        .failure()
        .stderr(contains("Run `kbase index`"));
}

#[test]
fn notes_tag_filter_shows_notes_with_tag() {
    let tmp = setup_vault();

    // Build the index first
    kbase(&tmp).arg("index").assert().success();

    // Test --tag filtering
    kbase(&tmp)
        .args(["notes", "--tag", "deep-dive"])
        .assert()
        .success()
        .stdout(contains("search-flow.md"))
        .stdout(contains("esql-analysis.md"));
}

#[test]
fn notes_tag_filter_with_files_flag() {
    let tmp = setup_vault();

    // Build the index first
    kbase(&tmp).arg("index").assert().success();

    // Test --tag with --files
    let out = kbase(&tmp)
        .args(["notes", "--tag", "wip", "--files"])
        .assert()
        .success();
    let stdout = String::from_utf8(out.get_output().stdout.clone()).unwrap();

    assert!(stdout.contains(".md"));
    // Should not contain titles when --files is used
    assert!(
        !stdout.contains("Search Flow"),
        "titles should be hidden with --files"
    );
}

#[test]
fn notes_tag_and_domain_combination() {
    let tmp = setup_vault();

    // Build the index first
    kbase(&tmp).arg("index").assert().success();

    // Test --tag with --domain (should show only lucene notes with deep-dive tag)
    let out = kbase(&tmp)
        .args(["notes", "--tag", "deep-dive", "--domain", "lucene"])
        .assert()
        .success();
    let stdout = String::from_utf8(out.get_output().stdout.clone()).unwrap();

    assert!(stdout.contains("search-flow.md"));
    assert!(!stdout.contains("esql-analysis.md")); // This is in elasticsearch domain
}

#[test]
fn notes_tag_not_found() {
    let tmp = setup_vault();

    // Build the index first
    kbase(&tmp).arg("index").assert().success();

    // Test nonexistent tag
    kbase(&tmp)
        .args(["notes", "--tag", "nonexistent-tag"])
        .assert()
        .success()
        .stdout(contains("No notes with tag 'nonexistent-tag'"));
}

#[test]
fn notes_tag_and_domain_no_results() {
    let tmp = setup_vault();

    // Build the index first
    kbase(&tmp).arg("index").assert().success();

    // Test combination that should yield no results
    kbase(&tmp)
        .args(["notes", "--tag", "rust", "--domain", "elasticsearch"])
        .assert()
        .success()
        .stdout(contains(
            "No notes in domain 'elasticsearch' with tag 'rust'",
        ));
}
