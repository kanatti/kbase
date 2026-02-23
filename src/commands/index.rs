use crate::{IndexType, links::LinkIndex, tags::TagIndex, vault::Vault};
use anyhow::Result;

pub fn handle_index(vault: &Vault, only: Vec<IndexType>) -> Result<()> {
    let index_dir = vault.index_dir()?;

    if only.is_empty() || only.contains(&IndexType::Tags) {
        println!("Building tag index...");
        let tag_index = TagIndex::build_from_vault(vault)?;
        vault.save_tag_index(&tag_index)?;

        let tag_count = tag_index.all_tags().len();
        println!("Built tag index: {} unique tags", tag_count);
        println!("Saved to {}", index_dir.join("tags.json").display());
    }

    if only.is_empty() || only.contains(&IndexType::Links) {
        println!("Building link index...");
        let (link_index, unresolved_count) = LinkIndex::build_from_vault(vault)?;
        link_index.save_to_json(&index_dir)?;

        println!("Built link index:");
        println!("  Saved to {}/links-forward.json", index_dir.display());
        println!("  Saved to {}/links-backward.json", index_dir.display());
        if unresolved_count > 0 {
            println!("  {} unresolved links (broken)", unresolved_count);
        }
    }

    if only.is_empty() || only.contains(&IndexType::Search) {
        println!("Search index not yet implemented");
    }

    Ok(())
}
