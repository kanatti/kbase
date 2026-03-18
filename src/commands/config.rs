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

pub fn handle_vaults(json: bool) -> Result<()> {
    let config = Config::load()?;

    if config.vaults.is_empty() {
        if json {
            output_json_vaults(&config)?;
        } else {
            println!("No vaults configured.");
            println!("Run `kbase add <name> <path>` to add a vault.");
        }
        return Ok(());
    }

    if json {
        output_json_vaults(&config)?;
    } else {
        config.print_vaults();
    }

    Ok(())
}

/// Output vaults as JSON
fn output_json_vaults(config: &Config) -> Result<()> {
    use serde_json::json;

    let cfg_path = config_path()?;
    let vaults: Vec<_> = config
        .vaults
        .iter()
        .map(|(name, vault_config)| {
            let active = name == &config.active_vault;
            json!({
                "name": name,
                "path": vault_config.path.to_string_lossy(),
                "active": active,
            })
        })
        .collect();

    let result = json!({
        "config_path": cfg_path.to_string_lossy(),
        "vaults": vaults,
        "count": config.vaults.len()
    });

    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
}

fn resolve_path(path: &str) -> Result<PathBuf> {
    let expanded = PathBuf::from(shellexpand::tilde(path).as_ref());
    if !expanded.exists() {
        bail!("Path does not exist: {}", expanded.display());
    }
    Ok(expanded)
}
