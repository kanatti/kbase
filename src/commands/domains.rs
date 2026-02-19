use crate::{SortBy, vault::Vault};
use anyhow::Result;

pub fn handle_domains(vault: &Vault, sort: SortBy) -> Result<()> {
    let mut domains = vault.domains()?;

    if sort == SortBy::Count {
        domains.sort_by(|a, b| b.note_count.cmp(&a.note_count));
    }

    if domains.is_empty() {
        println!("No domains found in vault.");
        return Ok(());
    }

    let max_name = domains.iter().map(|d| d.name.len()).max().unwrap_or(0);
    for d in &domains {
        let n = d.note_count;
        let label = if n == 1 { "note" } else { "notes" };
        println!("{:<width$}  {} {}", d.name, n, label, width = max_name);
    }

    Ok(())
}
