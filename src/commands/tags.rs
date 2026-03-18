use crate::{SortBy, output, vault::Vault};
use anyhow::Result;

pub fn handle_tags(vault: &Vault, sort: SortBy, json: bool) -> Result<()> {
    let Some(tag_index) = vault.load_tag_index()? else {
        if json {
            output_json_tags(&[], &sort)?;
        } else {
            println!("No tag index found. Run `kbase index` to build it first.");
        }
        return Ok(());
    };

    let tags = match sort {
        SortBy::Count => tag_index.all_tags_by_count(),
        SortBy::Name => tag_index.all_tags(),
    };

    if tags.is_empty() {
        if json {
            output_json_tags(&tags, &sort)?;
        } else {
            println!("No tags found.");
        }
        return Ok(());
    }

    if json {
        output_json_tags(&tags, &sort)?;
    } else {
        let rows: Vec<_> = tags
            .iter()
            .map(|(tag, count)| vec![tag.clone(), count.to_string()])
            .collect();

        output::print_table(&["Tag", "Notes"], &rows);
    }

    Ok(())
}

/// Output tags as JSON
fn output_json_tags(tags: &[(String, usize)], sort: &SortBy) -> Result<()> {
    use serde_json::json;

    let result = json!({
        "tags": tags.iter().map(|(name, count)| json!({
            "name": name,
            "notes": count,
        })).collect::<Vec<_>>(),
        "count": tags.len(),
        "sort": match sort {
            SortBy::Name => "name",
            SortBy::Count => "count",
        }
    });

    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
}
