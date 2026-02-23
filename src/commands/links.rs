use crate::links::LinkIndex;
use crate::vault::Vault;
use anyhow::{bail, Context, Result};
use std::path::PathBuf;

pub fn handle_links(
    vault: &Vault,
    note: String,
    forward: bool,
    backward: bool,
    json: bool,
) -> Result<()> {
    // Load the link index
    let index_dir = vault.index_dir()?;
    
    if !index_dir.join("links-forward.json").exists() {
        bail!("Link index not found. Run 'kbase index --only links' first.");
    }

    let link_index = LinkIndex::load_from_json(&index_dir)
        .context("Failed to load link index")?;

    // Normalize note path
    let note_path = PathBuf::from(&note);

    // Determine what to show based on flags
    let show_forward = forward || (!forward && !backward); // default to both if neither flag
    let show_backward = backward || (!forward && !backward);

    if json {
        output_json(&link_index, &note_path, show_forward, show_backward)?;
    } else {
        output_text(&link_index, &note_path, &note, show_forward, show_backward)?;
    }

    Ok(())
}

fn output_text(
    index: &LinkIndex,
    note_path: &PathBuf,
    note_display: &str,
    show_forward: bool,
    show_backward: bool,
) -> Result<()> {
    let forward_links = index.get_forward(note_path);
    let backward_links = index.get_backward(note_path);

    // Check if note has any links
    if forward_links.is_none() && backward_links.is_none() {
        println!("No links found for {}", note_display);
        return Ok(());
    }

    println!("Links for {}\n", note_display);

    if show_forward {
        if let Some(links) = forward_links {
            println!("Forward links ({}):", links.len());
            for link in links {
                println!("  {}", link.display());
            }
        } else {
            println!("Forward links (0):");
        }
        if show_backward {
            println!(); // spacing between sections
        }
    }

    if show_backward {
        if let Some(links) = backward_links {
            println!("Backward links ({}):", links.len());
            for link in links {
                println!("  {}", link.display());
            }
        } else {
            println!("Backward links (0):");
        }
    }

    Ok(())
}

fn output_json(
    index: &LinkIndex,
    note_path: &PathBuf,
    show_forward: bool,
    show_backward: bool,
) -> Result<()> {
    use serde_json::json;

    let mut result = json!({
        "note": note_path.to_string_lossy(),
        "depth": 1,
    });

    if show_forward {
        let links = index.get_forward(note_path);
        let forward_json = if let Some(links) = links {
            json!({
                "total": links.len(),
                "links": links.iter().map(|p| json!({
                    "path": p.to_string_lossy(),
                    "depth": 1
                })).collect::<Vec<_>>()
            })
        } else {
            json!({
                "total": 0,
                "links": []
            })
        };
        result["forward"] = forward_json;
    }

    if show_backward {
        let links = index.get_backward(note_path);
        let backward_json = if let Some(links) = links {
            json!({
                "total": links.len(),
                "links": links.iter().map(|p| json!({
                    "path": p.to_string_lossy(),
                    "depth": 1
                })).collect::<Vec<_>>()
            })
        } else {
            json!({
                "total": 0,
                "links": []
            })
        };
        result["backward"] = backward_json;
    }

    println!("{}", serde_json::to_string_pretty(&result)?);

    Ok(())
}
