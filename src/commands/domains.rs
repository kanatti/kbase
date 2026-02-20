use crate::{SortBy, output, vault::Vault};
use anyhow::Result;

pub fn handle_domains(vault: &Vault, sort: SortBy) -> Result<()> {
    let mut domains = vault.domains()?;

    match sort {
        SortBy::Count => domains.sort_by(|a, b| b.note_count.cmp(&a.note_count)),
        SortBy::Name => {} // Already sorted by name from vault.domains()
    }

    if domains.is_empty() {
        println!("No domains found in vault.");
        return Ok(());
    }

    let rows: Vec<_> = domains
        .iter()
        .map(|d| (d.name.clone(), d.note_count.to_string()))
        .collect();
    
    output::print_table(("Domain", "Notes"), &rows);

    Ok(())
}
