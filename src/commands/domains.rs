use crate::{SortBy, output, vault::Vault};
use anyhow::Result;

pub fn handle_domains(vault: &Vault, sort: SortBy, json: bool) -> Result<()> {
    let domains = vault.domains()?;

    if domains.is_empty() {
        if json {
            output_json_domains(&[], &sort)?;
        } else {
            println!("No domains found in vault.");
        }
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

    if json {
        output_json_domains(&sorted, &sort)?;
    } else {
        // Check if any domain has a description
        let has_descriptions = sorted.iter().any(|(_, _, desc)| desc.is_some());

        if has_descriptions {
            // Show 3-column table with descriptions
            let rows: Vec<_> = sorted
                .iter()
                .map(|(name, count, desc)| {
                    let desc_str = desc.as_deref().unwrap_or("").to_string();
                    vec![name.clone(), count.to_string(), desc_str]
                })
                .collect();

            output::print_table(&["Domain", "Notes", "Description"], &rows);
        } else {
            // Omit description column if none present
            let rows: Vec<_> = sorted
                .iter()
                .map(|(name, count, _)| vec![name.clone(), count.to_string()])
                .collect();

            output::print_table(&["Domain", "Notes"], &rows);
        }
    }

    Ok(())
}

/// Output domains as JSON
fn output_json_domains(
    domains: &[(String, usize, Option<String>)],
    sort: &SortBy,
) -> Result<()> {
    use serde_json::json;

    let result = json!({
        "domains": domains.iter().map(|(name, count, desc)| {
            let mut obj = json!({
                "name": name,
                "notes": count,
            });
            if let Some(d) = desc {
                obj["description"] = json!(d);
            }
            obj
        }).collect::<Vec<_>>(),
        "count": domains.len(),
        "sort": match sort {
            SortBy::Name => "name",
            SortBy::Count => "count",
        }
    });

    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
}
