// Command handlers

pub mod config;
pub mod domains;
pub mod notes;
pub mod read;
pub mod index;
pub mod tags;

use anyhow::Result;
use crate::vault::Vault;

// Re-export the command enum
pub use crate::Command;

/// Dispatch to appropriate command handler
pub fn handle_command(command: Command) -> Result<()> {
    match command {
        Command::Config => {
            config::handle_config()
        }
        
        Command::Add { name, path } => {
            config::handle_add(name, path)
        }
        
        Command::Use { name } => {
            config::handle_use(name)
        }
        
        Command::Vaults => {
            config::handle_vaults()
        }
        
        Command::Domains { sort } => {
            let vault = open_vault()?;
            domains::handle_domains(&vault, sort)
        }
        
        Command::Notes { domain, term, tag, files } => {
            let vault = open_vault()?;
            notes::handle_notes(&vault, domain, term, tag, files)
        }
        
        Command::Read { path, outline } => {
            let vault = open_vault()?;
            read::handle_read(&vault, path, outline)
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

/// Load vault from config file.
fn open_vault() -> Result<Vault> {
    let config = crate::config::Config::load()?;
    let (vault_name, vault_config) = config.get_active_vault()?;
    Vault::open(vault_config.path, vault_name)
}