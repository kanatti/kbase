use crate::{
    output,
    vault::{Note, Vault},
};
use anyhow::Result;

pub fn handle_notes(
    vault: &Vault,
    domain: Option<String>,
    term: Option<String>,
    tag: Option<String>,
    files: bool,
) -> Result<()> {
    if term.is_some() {
        eprintln!("--term search is not yet implemented");
        std::process::exit(1);
    }

    let notes = if let Some(ref tag_name) = tag {
        // Tag-first filtering approach
        get_notes_by_tag(vault, tag_name, domain.as_deref())?
    } else {
        // Original behavior: domain or all notes
        match &domain {
            Some(d) => vault.notes_in_domain(d)?,
            None => vault.all_notes()?,
        }
    };

    if notes.is_empty() {
        match (&domain, &tag) {
            (Some(d), Some(t)) => println!("No notes in domain '{}' with tag '{}'.", d, t),
            (None, Some(t)) => println!("No notes with tag '{}'.", t),
            (Some(d), None) => println!("No notes in domain '{}'.", d),
            (None, None) => println!("No notes found."),
        }
        return Ok(());
    }

    if files {
        for note in &notes {
            println!("{}", note.path.display());
        }
    } else {
        let rows: Vec<_> = notes
            .iter()
            .map(|n| (n.path.display().to_string(), n.title.clone()))
            .collect();

        output::print_table(("Path", "Title"), &rows);
    }

    Ok(())
}

/// Get notes by tag using tag-first filtering approach
fn get_notes_by_tag(vault: &Vault, tag: &str, domain: Option<&str>) -> Result<Vec<Note>> {
    // Load tag index
    let Some(tag_index) = vault.load_tag_index()? else {
        eprintln!("No tag index found. Run `kb index` to build it first.");
        std::process::exit(1);
    };

    // Get paths for the tag
    let tagged_paths = tag_index.notes_with_tag(tag);
    if tagged_paths.is_empty() {
        return Ok(Vec::new()); // No notes with this tag
    }

    // Convert paths to Notes, filtering by domain if specified
    let mut notes = Vec::new();
    for path_str in tagged_paths {
        // Apply domain filter if specified
        if let Some(domain_name) = domain {
            if !path_in_domain(&path_str, domain_name) {
                continue;
            }
        }

        // Convert path to Note
        if let Ok(note) = path_to_note(vault, &path_str) {
            notes.push(note);
        }
    }

    // Sort by filename for consistent output
    notes.sort_by(|a, b| a.filename.cmp(&b.filename));

    Ok(notes)
}

/// Check if a path belongs to the specified domain
fn path_in_domain(path: &str, domain: &str) -> bool {
    if let Some(slash_pos) = path.find('/') {
        let path_domain = &path[..slash_pos];
        path_domain == domain
    } else {
        // Root files have no domain
        false
    }
}

/// Convert a vault-relative path string to a Note struct
fn path_to_note(vault: &Vault, path_str: &str) -> Result<Note> {
    use std::path::PathBuf;

    let full_path = vault.root.join(path_str);

    if !full_path.exists() {
        anyhow::bail!("Note file not found: {}", path_str);
    }

    let filename = full_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(path_str)
        .to_string();

    let stem = full_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or(&filename)
        .to_string();

    let title = read_first_heading(&full_path).unwrap_or(stem);

    Ok(Note {
        path: PathBuf::from(path_str),
        filename,
        title,
    })
}

/// Read the first level-1 heading (`# Title`) from a file.
/// Only scans the first 20 lines for performance.
fn read_first_heading(path: &std::path::Path) -> Option<String> {
    let content = std::fs::read_to_string(path).ok()?;
    for line in content.lines().take(20) {
        if let Some(rest) = line.trim().strip_prefix("# ") {
            let title = rest.trim().to_string();
            if !title.is_empty() {
                return Some(title);
            }
        }
    }
    None
}
