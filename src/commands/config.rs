use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::{Result, bail};

use crate::config::{Config, VaultConfig, config_path};

pub fn handle_config() -> Result<()> {
    let path = config_path()?;

    if !path.exists() {
        println!("No config found.");
        println!("Run `kbase add <name> <path>` to add a vault.");
        return Ok(());
    }

    Config::load()?.print_summary()
}

pub fn handle_add(name: String, path: String) -> Result<()> {
    let vault_path = resolve_path(&path)?;

    // Load existing config or create new one
    let mut config = Config::load().unwrap_or_else(|_| Config {
        active_vault: name.clone(),
        vaults: HashMap::new(),
    });

    config
        .vaults
        .insert(name.clone(), VaultConfig { path: vault_path });

    // If this is the first vault, make it active
    if config.vaults.len() == 1 {
        config.active_vault = name.clone();
    }

    config.save()?;
    println!("Added vault '{}' to config", name);
    if config.active_vault == name {
        println!("Set as active vault");
    }

    Ok(())
}

pub fn handle_use(name: String) -> Result<()> {
    let mut config = Config::load()?;

    if !config.vaults.contains_key(&name) {
        let available: Vec<_> = config.vaults.keys().map(|s| s.as_str()).collect();
        bail!(
            "Vault '{}' not found. Available vaults: {}",
            name,
            available.join(", ")
        );
    }

    config.active_vault = name.clone();
    config.save()?;
    println!("Set '{}' as active vault", name);

    Ok(())
}

pub fn handle_vaults() -> Result<()> {
    let config = Config::load()?;

    if config.vaults.is_empty() {
        println!("No vaults configured.");
        println!("Run `kbase add <name> <path>` to add a vault.");
        return Ok(());
    }

    config.print_vaults();

    Ok(())
}

fn resolve_path(path: &str) -> Result<PathBuf> {
    let expanded = PathBuf::from(shellexpand::tilde(path).as_ref());
    if !expanded.exists() {
        bail!("Path does not exist: {}", expanded.display());
    }
    Ok(expanded)
}
