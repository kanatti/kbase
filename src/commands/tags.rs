use crate::{SortBy, output, vault::Vault};
use anyhow::Result;

pub fn handle_tags(vault: &Vault, sort: SortBy) -> Result<()> {
    let Some(tag_index) = vault.load_tag_index()? else {
        println!("No tag index found. Run `kbase index` to build it first.");
        return Ok(());
    };

    let tags = match sort {
        SortBy::Count => tag_index.all_tags_by_count(),
        SortBy::Name => tag_index.all_tags(),
    };

    if tags.is_empty() {
        println!("No tags found.");
        return Ok(());
    }

    let rows: Vec<_> = tags
        .iter()
        .map(|(tag, count)| (tag.clone(), count.to_string()))
        .collect();

    output::print_table(("Tag", "Notes"), &rows);

    Ok(())
}
