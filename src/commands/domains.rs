use crate::{SortBy, output, vault::Vault};
use anyhow::Result;

pub fn handle_domains(vault: &Vault, sort: SortBy) -> Result<()> {
    let domains = vault.domains()?;

    if domains.is_empty() {
        println!("No domains found in vault.");
        return Ok(());
    }

    // Extract descriptions for each domain and prepare for sorting
    let mut sorted: Vec<_> = domains
        .iter()
        .map(|d| {
            let description = vault.domain_description(&d.name);
            (d.name.clone(), d.note_count, description)
        })
        .collect();

    // Sort
    match sort {
        SortBy::Count => sorted.sort_by(|a, b| b.1.cmp(&a.1)),
        SortBy::Name => {} // Already sorted by name from vault.domains()
    }

    // Check if any domain has a description
    let has_descriptions = sorted.iter().any(|(_, _, desc)| desc.is_some());

    if has_descriptions {
        // Show 3-column table with descriptions
        let rows: Vec<_> = sorted
            .iter()
            .map(|(name, count, desc)| {
                let desc_str = desc.as_deref().unwrap_or("").to_string();
                (name.clone(), count.to_string(), desc_str)
            })
            .collect();

        output::print_table3(("Domain", "Notes", "Description"), &rows);
    } else {
        // Omit description column if none present
        let rows: Vec<_> = sorted
            .iter()
            .map(|(name, count, _)| (name.clone(), count.to_string()))
            .collect();

        output::print_table(("Domain", "Notes"), &rows);
    }

    Ok(())
}
