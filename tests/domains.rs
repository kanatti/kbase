mod common;

use assert_cmd::cargo::cargo_bin_cmd;
use common::{kbase, setup_vault};

#[test]
fn domains_lists_all() {
    let tmp = setup_vault();

    let output = String::from_utf8(
        kbase(&tmp)
            .arg("domains")
            .assert()
            .success()
            .get_output()
            .stdout
            .clone(),
    )
    .unwrap();

    let expected = "\
Domain         Notes  Description
elasticsearch  3      Distributed search and analytics engine built on top of Lucene.
lucene         5      Core full-text search library internals and algorithms used by Elasticsearch and Solr.
rust           1      
";

    assert_eq!(output, expected);
}

#[test]
fn domains_sorted_by_count() {
    let tmp = setup_vault();

    let output = String::from_utf8(
        kbase(&tmp)
            .args(["domains", "--sort", "count"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone(),
    )
    .unwrap();

    let expected = "\
Domain         Notes  Description
lucene         5      Core full-text search library internals and algorithms used by Elasticsearch and Solr.
elasticsearch  3      Distributed search and analytics engine built on top of Lucene.
rust           1      
";

    assert_eq!(output, expected);
}

#[test]
fn domains_count_includes_nested_files() {
    let tmp = setup_vault();

    let output = String::from_utf8(
        kbase(&tmp)
            .args(["notes", "--domain", "lucene", "--files"])
            .assert()
            .success()
            .get_output()
            .stdout
            .clone(),
    )
    .unwrap();

    // Should include nested files from lucene/indexing/
    assert!(output.contains("lucene/indexing/inverted-index.md"));
    assert!(output.contains("lucene/indexing/segment-merging.md"));

    // Should also include top-level files
    assert!(output.contains("lucene/search-flow.md"));
    assert!(output.contains("lucene/codecs.md"));
}

#[test]
fn domains_no_vault_shows_error() {
    let tmp = setup_vault();
    let mut cmd = cargo_bin_cmd!("kbase");
    cmd.env("KBASE_HOME", tmp.path().join(".kbase"));
    cmd.env_remove("KBASE_VAULT");

    let output = cmd.arg("domains").output().unwrap();

    let stderr = String::from_utf8(output.stderr).unwrap();
    assert_eq!(
        stderr,
        "Error: No config found. Run `kbase config add <name> <path>` to add a vault.\n"
    );
}
