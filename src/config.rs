use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

const CONFIG_DIR: &str = ".kb";
const CONFIG_FILE: &str = "config.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub active_vault: String,
    #[serde(default)]
    pub vaults: HashMap<String, VaultConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VaultConfig {
    pub path: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = config_path()?;

        if !path.exists() {
            bail!("No config found. Run `kb config add <name> <path>` to add a vault.");
        }

        let contents = fs::read_to_string(&path).context("Could not read config file")?;
        let config: Config = toml::from_str(&contents).context("Invalid config format")?;

        Ok(config)
    }

    pub fn get_active_vault(&self) -> Result<(String, VaultConfig)> {
        match self.vaults.get(&self.active_vault) {
            Some(vault_config) => Ok((self.active_vault.clone(), vault_config.clone())),
            None => bail!("Active vault '{}' not found in config", self.active_vault),
        }
    }

    pub fn save(&self) -> Result<()> {
        let path = config_path()?;
        fs::create_dir_all(path.parent().unwrap()).context("Could not create config directory")?;

        let contents = toml::to_string(self).context("Could not serialize config")?;
        fs::write(&path, contents).context("Could not write config file")?;

        Ok(())
    }
}

pub fn show() -> Result<()> {
    let path = config_path()?;

    if !path.exists() {
        println!("No config found.");
        println!("Run `kb config add <name> <path>` to add a vault.");
        return Ok(());
    }

    let config = Config::load()?;
    println!("Config: {}", path.display());
    println!("active_vault = {}", config.active_vault);
    println!("\nVaults:");
    for (name, vault_config) in &config.vaults {
        let marker = if name == &config.active_vault {
            " (active)"
        } else {
            ""
        };
        println!("  {} = {}{}", name, vault_config.path.display(), marker);
    }

    Ok(())
}

pub fn add_vault(name: &str, path: &str) -> Result<()> {
    let vault_path = resolve_path(path)?;

    // Load existing config or create new one
    let mut config = Config::load().unwrap_or_else(|_| Config {
        active_vault: name.to_string(),
        vaults: HashMap::new(),
    });

    // Add the vault
    config
        .vaults
        .insert(name.to_string(), VaultConfig { path: vault_path });

    // If this is the first vault, make it active
    if config.vaults.len() == 1 {
        config.active_vault = name.to_string();
    }

    config.save()?;
    println!("Added vault '{}' to config", name);
    if config.active_vault == name {
        println!("Set as active vault");
    }

    Ok(())
}

pub fn set_active_vault(name: &str) -> Result<()> {
    let mut config = Config::load()?;

    if !config.vaults.contains_key(name) {
        let available_vaults: Vec<&String> = config.vaults.keys().collect();
        bail!(
            "Vault '{}' not found. Available vaults: {}",
            name,
            available_vaults
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    config.active_vault = name.to_string();
    config.save()?;
    println!("Set '{}' as active vault", name);

    Ok(())
}

pub fn list_vaults() -> Result<()> {
    let config = Config::load()?;

    if config.vaults.is_empty() {
        println!("No vaults configured.");
        println!("Run `kb config add <name> <path>` to add a vault.");
        return Ok(());
    }

    println!("Configured vaults:");
    for (name, vault_config) in &config.vaults {
        let marker = if name == &config.active_vault {
            " (active)"
        } else {
            ""
        };
        println!("  {}{} -> {}", name, marker, vault_config.path.display());
    }

    Ok(())
}

fn config_path() -> Result<PathBuf> {
    let kb_dir = if let Ok(dir) = std::env::var("KB_CONFIG_DIR") {
        // KB_CONFIG_DIR points directly to the .kb directory
        PathBuf::from(dir).join(CONFIG_DIR)
    } else {
        // Default: ~/.kb
        dirs::home_dir()
            .context("Could not find home directory")?
            .join(CONFIG_DIR)
    };

    Ok(kb_dir.join(CONFIG_FILE))
}

fn resolve_path(path: &str) -> Result<PathBuf> {
    let expanded = PathBuf::from(shellexpand::tilde(path).as_ref());

    if !expanded.exists() {
        bail!("Path does not exist: {}", expanded.display());
    }

    Ok(expanded)
}
