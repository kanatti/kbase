mod common;

use assert_cmd::cargo::cargo_bin_cmd;
use common::{kb, setup_vault};

#[test]
fn domains_lists_all() {
    let tmp = setup_vault();

    let output = String::from_utf8(
        kb(&tmp)
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
elasticsearch  2      Distributed search and analytics engine built on top of Lucene.
lucene         3      Core full-text search library internals and algorithms used by Elasticsearch and Solr.
rust           1      
";

    assert_eq!(output, expected);
}

#[test]
fn domains_sorted_by_count() {
    let tmp = setup_vault();

    let output = String::from_utf8(
        kb(&tmp)
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
lucene         3      Core full-text search library internals and algorithms used by Elasticsearch and Solr.
elasticsearch  2      Distributed search and analytics engine built on top of Lucene.
rust           1      
";

    assert_eq!(output, expected);
}

#[test]
fn domains_no_vault_shows_error() {
    let tmp = setup_vault();
    let mut cmd = cargo_bin_cmd!("kb");
    cmd.env("KB_HOME", tmp.path().join(".kb"));
    cmd.env_remove("KB_VAULT");

    let output = cmd.arg("domains").output().unwrap();

    let stderr = String::from_utf8(output.stderr).unwrap();
    assert_eq!(
        stderr,
        "Error: No config found. Run `kb config add <name> <path>` to add a vault.\n"
    );
}
