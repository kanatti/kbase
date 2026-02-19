#![allow(dead_code)]

use assert_cmd::Command;
use assert_cmd::cargo::cargo_bin_cmd;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Absolute path to the checked-in fixture vault.
pub fn fixture_vault() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/vault")
}

/// Copy the fixture vault into a fresh TempDir and return it.
/// Each test gets its own isolated copy — safe for write operations too.
pub fn setup_vault() -> TempDir {
    let tmp = TempDir::new().unwrap();
    copy_dir(&fixture_vault(), tmp.path()).unwrap();
    tmp
}

/// Build a `kb` command with a properly configured vault in the temp directory.
pub fn kb(tmp: &TempDir) -> Command {
    let mut cmd = cargo_bin_cmd!("kb");

    // Set KB_HOME to point directly to the .kb dir inside the temp directory
    cmd.env("KB_HOME", tmp.path().join(".kb"));

    // Create a proper config file for the vault
    setup_vault_config(tmp);

    cmd
}

/// Set up a config file in the temp directory for the test vault.
fn setup_vault_config(tmp: &TempDir) {
    use std::fs;

    // Create the config directory
    let config_dir = tmp.path().join(".kb");
    fs::create_dir_all(&config_dir).unwrap();

    // Create config.toml with proper vault configuration format
    let vault_path = tmp.path().to_string_lossy().replace("\\", "\\\\"); // Handle Windows paths
    let config_content = format!(
        r#"active_vault = "test-vault"

[vaults.test-vault]
path = "{}"
"#,
        vault_path
    );

    let config_path = config_dir.join("config.toml");
    fs::write(&config_path, config_content).unwrap();
}

fn copy_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let dst_path = dst.join(entry.file_name());
        if entry.path().is_dir() {
            fs::create_dir_all(&dst_path)?;
            copy_dir(&entry.path(), &dst_path)?;
        } else {
            fs::copy(&entry.path(), &dst_path)?;
        }
    }
    Ok(())
}
