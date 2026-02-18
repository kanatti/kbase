use anyhow::Result;
use crate::vault::Vault;
use crate::tags::TagIndex;

pub fn handle_tags(vault: &Vault, sort: String) -> Result<()> {
    // Load the tag index
    let index_dir = get_index_dir(vault)?;
    let tags_json_path = index_dir.join("tags.json");
    
    if !tags_json_path.exists() {
        println!("No tag index found. Run `kb index` to build it first.");
        return Ok(());
    }
    
    let tag_index = TagIndex::load_from_json(&tags_json_path)?;
    
    let tags = match sort.as_str() {
        "count" => tag_index.all_tags_by_count(),
        _ => tag_index.all_tags(),
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
    let kb_dir = if let Ok(dir) = std::env::var("KB_CONFIG_DIR") {
        std::path::PathBuf::from(dir)
    } else {
        dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?
    }.join(".kb");
    
    Ok(kb_dir.join("indexes").join(&vault.name))
}