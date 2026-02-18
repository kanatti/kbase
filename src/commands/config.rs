use anyhow::Result;

pub fn handle_config() -> Result<()> {
    crate::config::show()
}

/// Handle `kb add vault-name /path` command
pub fn handle_add(name: String, path: String) -> Result<()> {
    crate::config::add_vault(&name, &path)
}

/// Handle `kb use vault-name` command
pub fn handle_use(name: String) -> Result<()> {
    crate::config::set_active_vault(&name)
}

/// Handle `kb vaults` command
pub fn handle_vaults() -> Result<()> {
    crate::config::list_vaults()
}