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
    /// Local paths for repositories (key = repo name, value = local path)
    #[serde(default)]
    pub repos: HashMap<String, PathBuf>,
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

    /// Get the local path for a repository in the active vault.
    pub fn get_repo_path(&self, repo_name: &str) -> Option<PathBuf> {
        let (_, vault_config) = self.get_active_vault().ok()?;
        vault_config.repos.get(repo_name).cloned()
    }

    /// Set the local path for a repository in the active vault.
    pub fn set_repo_path(&mut self, repo_name: String, path: PathBuf) -> Result<()> {
        let vault_config = self.vaults.get_mut(&self.active_vault)
            .ok_or_else(|| anyhow::anyhow!("Active vault '{}' not found", self.active_vault))?;
        
        vault_config.repos.insert(repo_name, path);
        Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_repo_path_none() {
        let config = Config {
            active_vault: "test".to_string(),
            vaults: HashMap::from([
                ("test".to_string(), VaultConfig {
                    path: PathBuf::from("/tmp"),
                    repos: HashMap::new(),
                }),
            ]),
        };

        assert_eq!(config.get_repo_path("datafusion"), None);
    }

    #[test]
    fn test_get_repo_path_some() {
        let config = Config {
            active_vault: "test".to_string(),
            vaults: HashMap::from([
                ("test".to_string(), VaultConfig {
                    path: PathBuf::from("/tmp"),
                    repos: HashMap::from([
                        ("datafusion".to_string(), PathBuf::from("/code/datafusion")),
                    ]),
                }),
            ]),
        };

        assert_eq!(
            config.get_repo_path("datafusion"),
            Some(PathBuf::from("/code/datafusion"))
        );
    }

    #[test]
    fn test_set_repo_path() {
        let mut config = Config {
            active_vault: "test".to_string(),
            vaults: HashMap::from([
                ("test".to_string(), VaultConfig {
                    path: PathBuf::from("/tmp"),
                    repos: HashMap::new(),
                }),
            ]),
        };

        config.set_repo_path("datafusion".to_string(), PathBuf::from("/code/datafusion")).unwrap();

        assert_eq!(
            config.get_repo_path("datafusion"),
            Some(PathBuf::from("/code/datafusion"))
        );
    }
}
