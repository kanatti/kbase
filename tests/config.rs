use assert_cmd::Command;
use assert_cmd::cargo::cargo_bin_cmd;
use predicates::str::contains;
use tempfile::TempDir;

fn kbase(tmp: &TempDir) -> Command {
    let mut cmd = cargo_bin_cmd!("kbase");
    cmd.env("KBASE_HOME", tmp.path().join(".kbase"));
    cmd
}

#[test]
fn config_show_no_config() {
    let tmp = TempDir::new().unwrap();
    kbase(&tmp)
        .arg("config")
        .assert()
        .success()
        .stdout(contains("No config found"));
}

#[test]
fn add_vault() {
    let tmp = TempDir::new().unwrap();

    kbase(&tmp)
        .args(["add", "test-vault", tmp.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(contains("Added vault 'test-vault' to config"));
}

#[test]
fn config_show_after_add() {
    let tmp = TempDir::new().unwrap();

    // Add a vault first
    kbase(&tmp)
        .args(["add", "test-vault", tmp.path().to_str().unwrap()])
        .assert()
        .success();

    // Show config
    kbase(&tmp)
        .arg("config")
        .assert()
        .success()
        .stdout(contains("✔ test-vault"));
}

#[test]
fn use_vault_not_found() {
    let tmp = TempDir::new().unwrap();

    // Try to use vault that doesn't exist
    kbase(&tmp)
        .args(["use", "nonexistent"])
        .assert()
        .failure()
        .stderr(contains("No config found"));
}

#[test]
fn add_and_use_vault() {
    let tmp = TempDir::new().unwrap();

    // Add two vaults
    kbase(&tmp)
        .args(["add", "vault1", tmp.path().to_str().unwrap()])
        .assert()
        .success();

    kbase(&tmp)
        .args(["add", "vault2", tmp.path().to_str().unwrap()])
        .assert()
        .success();

    // Switch active vault
    kbase(&tmp)
        .args(["use", "vault2"])
        .assert()
        .success()
        .stdout(contains("Set 'vault2' as active vault"));
}

#[test]
fn list_vaults() {
    let tmp = TempDir::new().unwrap();

    // Add a vault
    kbase(&tmp)
        .args(["add", "test-vault", tmp.path().to_str().unwrap()])
        .assert()
        .success();

    // List vaults
    kbase(&tmp)
        .args(["vaults"])
        .assert()
        .success()
        .stdout(contains("test-vault"))
        .stdout(contains("✔"));
}

#[test]
fn kbase_vault_env_var_override() {
    let tmp = TempDir::new().unwrap();

    // Add two vaults
    kbase(&tmp)
        .args(["add", "vault1", tmp.path().to_str().unwrap()])
        .assert()
        .success();

    kbase(&tmp)
        .args(["add", "vault2", tmp.path().to_str().unwrap()])
        .assert()
        .success();

    // Use vault1 as active
    kbase(&tmp).args(["use", "vault1"]).assert().success();

    // Override with KBASE_VAULT env var to use vault2 temporarily
    kbase(&tmp)
        .env("KBASE_VAULT", "vault2")
        .args(["domains"])
        .assert()
        .success();
}

#[test]
fn kbase_vault_env_var_invalid_vault() {
    let tmp = TempDir::new().unwrap();

    // Add a vault
    kbase(&tmp)
        .args(["add", "test-vault", tmp.path().to_str().unwrap()])
        .assert()
        .success();

    // Try to use nonexistent vault via KBASE_VAULT
    kbase(&tmp)
        .env("KBASE_VAULT", "nonexistent")
        .args(["domains"])
        .assert()
        .failure()
        .stderr(contains("Vault 'nonexistent' not found in config"))
        .stderr(contains("Available vaults: test-vault"));
}
