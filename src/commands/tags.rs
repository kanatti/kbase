use crate::{SortBy, tags::TagIndex, vault::Vault};
use anyhow::Result;

pub fn handle_tags(vault: &Vault, sort: SortBy) -> Result<()> {
    // Load the tag index
    let index_dir = get_index_dir(vault)?;
    let tags_json_path = index_dir.join("tags.json");

    if !tags_json_path.exists() {
        println!("No tag index found. Run `kb index` to build it first.");
        return Ok(());
    }

    let tag_index = TagIndex::load_from_json(&tags_json_path)?;

    let tags = match sort {
        SortBy::Count => tag_index.all_tags_by_count(),
        SortBy::Name => tag_index.all_tags(),
    };

    if tags.is_empty() {
        println!("No tags found. Run `kb index` to build the tag index.");
        return Ok(());
    }

    for (tag, count) in tags {
        let plural = if count == 1 { "note" } else { "notes" };
        println!("{} ({} {})", tag, count, plural);
    }

    Ok(())
}

fn get_index_dir(vault: &Vault) -> Result<std::path::PathBuf> {
    Ok(crate::config::kb_home()?.join("indexes").join(&vault.name))
}
