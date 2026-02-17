mod common;

use assert_cmd::cargo::cargo_bin_cmd;
use common::{kb, setup_vault};
use predicates::prelude::PredicateBooleanExt;
use predicates::str::contains;

#[test]
fn topics_lists_topic_dirs() {
    let tmp = setup_vault();

    kb(&tmp)
        .arg("topics")
        .assert()
        .success()
        .stdout(contains("elasticsearch"))
        .stdout(contains("lucene"))
        .stdout(contains("rust"));
}

#[test]
fn topics_shows_note_counts() {
    let tmp = setup_vault();

    let out = kb(&tmp).arg("topics").assert().success();
    let stdout = String::from_utf8(out.get_output().stdout.clone()).unwrap();

    let es_line = stdout.lines().find(|l| l.contains("elasticsearch")).unwrap();
    assert!(es_line.contains('2'), "expected 2 notes for elasticsearch, got: {es_line}");

    let lucene_line = stdout.lines().find(|l| l.contains("lucene")).unwrap();
    assert!(lucene_line.contains('3'), "expected 3 notes for lucene, got: {lucene_line}");

    let rust_line = stdout.lines().find(|l| l.contains("rust")).unwrap();
    assert!(rust_line.contains('1'), "expected 1 note for rust, got: {rust_line}");
}

#[test]
fn topics_excludes_underscore_dirs() {
    let tmp = setup_vault();

    kb(&tmp)
        .arg("topics")
        .assert()
        .success()
        .stdout(contains("__templates").not())
        .stdout(contains("_logs").not());
}

#[test]
fn topics_sorted_by_name_default() {
    let tmp = setup_vault();

    let out = kb(&tmp).arg("topics").assert().success();
    let stdout = String::from_utf8(out.get_output().stdout.clone()).unwrap();
    let names: Vec<&str> = stdout
        .lines()
        .map(|l| l.trim().split_whitespace().next().unwrap_or(""))
        .filter(|s| !s.is_empty())
        .collect();

    let mut sorted = names.clone();
    sorted.sort();
    assert_eq!(names, sorted, "topics should be sorted alphabetically");
}

#[test]
fn topics_sorted_by_count() {
    let tmp = setup_vault();

    let out = kb(&tmp)
        .args(["topics", "--sort", "count"])
        .assert()
        .success();
    let stdout = String::from_utf8(out.get_output().stdout.clone()).unwrap();

    // lucene (3) > elasticsearch (2) > rust (1)
    let lucene_pos = stdout.find("lucene").unwrap();
    let es_pos = stdout.find("elasticsearch").unwrap();
    let rust_pos = stdout.find("rust").unwrap();

    assert!(lucene_pos < es_pos, "lucene (3) should come before elasticsearch (2)");
    assert!(es_pos < rust_pos, "elasticsearch (2) should come before rust (1)");
}

#[test]
fn topics_no_vault_shows_error() {
    let tmp = setup_vault();
    let mut cmd = cargo_bin_cmd!("kb");
    cmd.env("KB_CONFIG_DIR", tmp.path());
    cmd.env_remove("KB_VAULT");

    cmd.arg("topics").assert().failure().stderr(contains("Error"));
}
