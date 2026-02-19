use crate::{IndexType, tags, vault::Vault};
use anyhow::Result;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub fn handle_index(vault: &Vault, only: Vec<IndexType>) -> Result<()> {
    let build_all = only.is_empty();
    let build_tags = build_all || only.contains(&IndexType::Tags);

    // Determine index directory
    let index_dir = get_index_dir(vault)?;

    if build_tags {
        println!("Building tag index...");
        build_tag_index(vault, &index_dir)?;
    }

    // Future: build other indexes here
    if build_all || only.contains(&IndexType::Links) {
        println!("Link index not yet implemented");
    }
    if build_all || only.contains(&IndexType::Search) {
        println!("Search index not yet implemented");
    }

    Ok(())
}

fn get_index_dir(vault: &Vault) -> Result<PathBuf> {
    Ok(crate::config::kb_home()?.join("indexes").join(&vault.name))
}

fn build_tag_index(vault: &Vault, index_dir: &Path) -> Result<()> {
    let mut tag_accumulator: HashMap<String, Vec<String>> = HashMap::new();

    // Scan all notes in the vault
    let all_notes = vault.all_notes()?;
    println!("Scanning {} notes...", all_notes.len());

    for note in &all_notes {
        // Read note content (note.path is relative to vault root)
        let full_path = vault.root.join(&note.path);
        let content = std::fs::read_to_string(&full_path)?;

        // Extract tags from content
        let tags = tags::extract_tags(&content);

        if !tags.is_empty() {
            // note.path is already relative to vault root
            let relative_path = note.path.to_string_lossy().to_string();

            // Accumulate tags
            for tag in tags {
                tag_accumulator
                    .entry(tag)
                    .or_insert_with(Vec::new)
                    .push(relative_path.clone());
            }
        }
    }

    // Build TagIndex and save
    let tag_index = tags::TagIndex::from_tag_map(tag_accumulator);
    let tags_json_path = index_dir.join("tags.json");

    tag_index.save_to_json(&tags_json_path)?;

    let tag_count = tag_index.all_tags().len();
    println!("Built tag index: {} unique tags", tag_count);
    println!("Saved to {}", tags_json_path.display());

    Ok(())
}
