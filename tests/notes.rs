mod common;

use common::{kb, setup_vault};
use predicates::str::contains;

#[test]
fn notes_lists_all_notes_without_topic() {
    let tmp = setup_vault();

    kb(&tmp)
        .arg("notes")
        .assert()
        .success()
        .stdout(contains("elasticsearch"))
        .stdout(contains("lucene"))
        .stdout(contains("rust"));
}

#[test]
fn notes_topic_filter_lists_notes_in_topic() {
    let tmp = setup_vault();

    kb(&tmp)
        .args(["notes", "--topic", "lucene"])
        .assert()
        .success()
        .stdout(contains("01-home.md"))
        .stdout(contains("search-flow.md"))
        .stdout(contains("codecs.md"));
}

#[test]
fn notes_topic_filter_shows_titles() {
    let tmp = setup_vault();

    kb(&tmp)
        .args(["notes", "--topic", "lucene"])
        .assert()
        .success()
        .stdout(contains("Lucene"))
        .stdout(contains("Search Flow Deep Dive"));
}

#[test]
fn notes_topic_filter_falls_back_to_stem_when_no_heading() {
    let tmp = setup_vault();

    kb(&tmp)
        .args(["notes", "--topic", "lucene"])
        .assert()
        .success()
        .stdout(contains("codecs"));
}

#[test]
fn notes_files_flag_hides_titles() {
    let tmp = setup_vault();

    let out = kb(&tmp)
        .args(["notes", "--topic", "lucene", "--files"])
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
fn notes_unknown_topic_gives_error() {
    let tmp = setup_vault();

    kb(&tmp)
        .args(["notes", "--topic", "nonexistent"])
        .assert()
        .failure()
        .stderr(contains("Error"));
}

#[test]
fn notes_term_not_yet_implemented() {
    let tmp = setup_vault();

    kb(&tmp)
        .args(["notes", "--term", "search"])
        .assert()
        .failure()
        .stderr(contains("not yet implemented"));
}
