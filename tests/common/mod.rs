#![allow(dead_code)]

use assert_cmd::Command;
use assert_cmd::cargo::cargo_bin_cmd;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// Absolute path to the checked-in fixture vault.
pub fn fixture_vault() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/vault")
}

/// Copy the fixture vault into a fresh TempDir and return it.
/// Each test gets its own isolated copy — safe for write operations too.
pub fn setup_vault() -> TempDir {
    let tmp = TempDir::new().unwrap();
    copy_dir(&fixture_vault(), tmp.path()).unwrap();
    tmp
}

/// Build a `kb` command with config dir and vault both pointed at `tmp`.
pub fn kb(tmp: &TempDir) -> Command {
    let mut cmd = cargo_bin_cmd!("kb");
    cmd.env("KB_CONFIG_DIR", tmp.path());
    cmd.env("KB_VAULT", tmp.path());
    cmd
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
