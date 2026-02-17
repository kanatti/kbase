use assert_cmd::Command;
use assert_cmd::cargo::cargo_bin_cmd;
use predicates::str::contains;
use tempfile::TempDir;

fn kb(tmp: &TempDir) -> Command {
    let mut cmd = cargo_bin_cmd!("kb");
    cmd.env("KB_CONFIG_DIR", tmp.path());
    cmd
}

#[test]
fn config_show_no_config() {
    let tmp = TempDir::new().unwrap();
    kb(&tmp)
        .arg("config")
        .assert()
        .success()
        .stdout(contains("No config found"));
}

#[test]
fn config_set_vault() {
    let tmp = TempDir::new().unwrap();
    kb(&tmp)
        .args(["config", "set", "vault", tmp.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(contains("Config written to"));
}

#[test]
fn config_show_after_set() {
    let tmp = TempDir::new().unwrap();
    kb(&tmp)
        .args(["config", "set", "vault", tmp.path().to_str().unwrap()])
        .assert()
        .success();

    kb(&tmp)
        .arg("config")
        .assert()
        .success()
        .stdout(contains("vault"));
}

#[test]
fn config_set_unknown_key() {
    let tmp = TempDir::new().unwrap();
    kb(&tmp)
        .args(["config", "set", "unknown", "value"])
        .assert()
        .failure()
        .stderr(contains("Unknown config key"));
}
