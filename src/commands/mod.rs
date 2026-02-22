// Command handlers

pub mod config;
pub mod domains;
pub mod index;
pub mod notes;
pub mod read;
pub mod tags;

use crate::config::Config;
use crate::vault::Vault;
use anyhow::Result;
use std::env;

// Re-export the command enum
pub use crate::Command;

/// Dispatch to appropriate command handler
pub fn handle_command(command: Command) -> Result<()> {
    match command {
        Command::Config => config::handle_config(),
        Command::Add { name, path } => config::handle_add(name, path),
        Command::Use { name } => config::handle_use(name),
        Command::Vaults => config::handle_vaults(),
        Command::Domains { sort } => {
            let vault = open_vault()?;
            domains::handle_domains(&vault, sort)
        }
        Command::Notes {
            domain,
            term,
            tag,
            files,
        } => {
            let vault = open_vault()?;
            notes::handle_notes(&vault, domain, term, tag, files)
        }
        Command::Read {
            path,
            outline,
            line_numbers,
        } => {
            let vault = open_vault()?;
            read::handle_read(&vault, path, outline, line_numbers)
        }
        Command::Tags { sort } => {
            let vault = open_vault()?;
            tags::handle_tags(&vault, sort)
        }
        Command::Index { only } => {
            let vault = open_vault()?;
            index::handle_index(&vault, only)
        }
    }
}

/// Load vault from config file, with optional KBASE_VAULT override.
fn open_vault() -> Result<Vault> {
    let config = Config::load()?;

    // Check KBASE_VAULT environment variable first (vault name)
    if let Ok(vault_name) = env::var("KBASE_VAULT") {
        if let Some(vault_config) = config.vaults.get(&vault_name) {
            return Vault::open(vault_config.path.clone(), vault_name);
        } else {
            let available: Vec<_> = config.vaults.keys().map(|s| s.as_str()).collect();
            anyhow::bail!(
                "Vault '{}' not found in config. Available vaults: {}",
                vault_name,
                available.join(", ")
            );
        }
    }

    // Fall back to active_vault from config
    let (vault_name, vault_config) = config.get_active_vault()?;
    Vault::open(vault_config.path, vault_name)
}
