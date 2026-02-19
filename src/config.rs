use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

const DEFAULT_KB_HOME: &str = ".kb";
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

    /// Save config to disk.
    pub fn save(&self) -> Result<()> {
        let path = config_path()?;
        fs::create_dir_all(path.parent().unwrap()).context("Could not create config directory")?;

        let contents = toml::to_string(self).context("Could not serialize config")?;
        fs::write(&path, contents).context("Could not write config file")?;

        Ok(())
    }
}

/// Returns the kb home directory (~/.kb by default, or $KB_HOME if set).
/// $KB_HOME points directly to the kb home dir (e.g. /tmp/test/.kb in tests).
pub fn kb_home() -> Result<PathBuf> {
    if let Ok(dir) = std::env::var("KB_HOME") {
        return Ok(PathBuf::from(dir));
    }
    let home = dirs::home_dir().context("Could not find home directory")?;
    Ok(home.join(DEFAULT_KB_HOME))
}

/// Returns the path to the config file (~/.kb/config.toml by default).
pub fn config_path() -> Result<PathBuf> {
    Ok(kb_home()?.join(CONFIG_FILE))
}
