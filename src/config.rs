use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

const CONFIG_DIR: &str = ".kb";
const CONFIG_FILE: &str = "config.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub vault: PathBuf,
}

impl Config {
    pub fn load() -> Result<Self> {
        // --vault flag and KB_VAULT env var are handled in main before calling load()
        let path = config_path()?;

        if !path.exists() {
            bail!("No config found. Run `kb config set vault /path/to/vault` to get started.");
        }

        let contents = fs::read_to_string(&path).context("Could not read config file")?;
        let config: Config = toml::from_str(&contents).context("Invalid config format")?;

        Ok(config)
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
        println!("Run `kb config set vault /path/to/vault` to get started.");
        return Ok(());
    }

    let config = Config::load()?;
    println!("Config: {}", path.display());
    println!("vault = {}", config.vault.display());

    Ok(())
}

pub fn set(key: &str, value: &str) -> Result<()> {
    match key {
        "vault" => {
            let vault = resolve_path(value)?;
            // Load existing config or start fresh
            let mut config = Config::load().unwrap_or(Config { vault: vault.clone() });
            config.vault = vault;
            config.save()?;
            println!("Config written to {}", config_path()?.display());
        }
        _ => bail!("Unknown config key: {key}"),
    }

    Ok(())
}

fn config_path() -> Result<PathBuf> {
    let dir = if let Ok(dir) = std::env::var("KB_CONFIG_DIR") {
        PathBuf::from(dir)
    } else {
        dirs::home_dir()
            .context("Could not find home directory")?
            .join(CONFIG_DIR)
    };

    Ok(dir.join(CONFIG_FILE))
}

fn resolve_path(path: &str) -> Result<PathBuf> {
    let expanded = PathBuf::from(shellexpand::tilde(path).as_ref());

    if !expanded.exists() {
        bail!("Path does not exist: {}", expanded.display());
    }

    Ok(expanded)
}
