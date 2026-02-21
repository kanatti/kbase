use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

use crate::output;

const DEFAULT_KBASE_HOME: &str = ".kbase";
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
    /// Load config from disk.
    pub fn load() -> Result<Self> {
        let path = config_path()?;

        if !path.exists() {
            bail!("No config found. Run `kbase config add <name> <path>` to add a vault.");
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

    /// Save config to disk.
    pub fn save(&self) -> Result<()> {
        let path = config_path()?;
        fs::create_dir_all(path.parent().unwrap()).context("Could not create config directory")?;

        let contents = toml::to_string(self).context("Could not serialize config")?;
        fs::write(&path, contents).context("Could not write config file")?;

        Ok(())
    }

    /// Print a summary of the config.
    pub fn print_summary(&self) -> Result<()> {
        let path = config_path()?;
        println!("Config: {}", path.display());
        println!();
        self.print_vaults();
        Ok(())
    }

    /// Print all configured vaults.
    pub fn print_vaults(&self) {
        let rows: Vec<_> = self
            .vaults
            .iter()
            .map(|(name, vault_config)| {
                let marker = if name == &self.active_vault {
                    "✔ "
                } else {
                    "  "
                };
                let left = format!("{}{}", marker, name);
                let right = vault_config.path.display().to_string();
                (left, right)
            })
            .collect();

        output::print_table(("Vault", "Path"), &rows);
    }
}

/// Returns the kbase home directory (~/.kbase by default, or $KBASE_HOME if set).
/// $KBASE_HOME points directly to the kbase home dir (e.g. /tmp/test/.kbase in tests).
pub fn kbase_home() -> Result<PathBuf> {
    if let Ok(dir) = std::env::var("KBASE_HOME") {
        return Ok(PathBuf::from(dir));
    }
    let home = dirs::home_dir().context("Could not find home directory")?;
    Ok(home.join(DEFAULT_KBASE_HOME))
}

/// Returns the path to the config file (~/.kbase/config.toml by default).
pub fn config_path() -> Result<PathBuf> {
    Ok(kbase_home()?.join(CONFIG_FILE))
}
