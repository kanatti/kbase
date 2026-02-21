use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

use crate::config::kb_home;
use crate::domains;
use crate::tags::TagIndex;

/// An open markdown vault rooted at a filesystem path.
pub struct Vault {
    pub root: PathBuf,
    pub name: String,
}

/// A top-level domain folder inside a vault.
pub struct Domain {
    pub name: String,
    pub note_count: usize,
}

pub struct Note {
    /// Path relative to vault root (e.g. "lucene/search-flow.md")
    pub path: PathBuf,
    /// Just the filename (e.g. "search-flow.md")
    pub filename: String,
    /// First # heading in the file, or filename stem if none
    pub title: String,
}

impl Vault {
    pub fn open(root: PathBuf, name: String) -> Result<Self> {
        validate_dir(&root, "Vault path")?;
        Ok(Vault { root, name })
    }

    /// Get the directory where indexes for this vault are stored.
    pub fn index_dir(&self) -> Result<PathBuf> {
        Ok(kb_home()?.join(&self.name))
    }

    /// Load the tag index for this vault.
    /// Returns None if the index hasn't been built yet.
    pub fn load_tag_index(&self) -> Result<Option<TagIndex>> {
        let path = self.index_dir()?.join("tags.json");
        if !path.exists() {
            return Ok(None);
        }
        Ok(Some(TagIndex::load_from_json(&path)?))
    }

    /// Save the tag index for this vault.
    pub fn save_tag_index(&self, index: &TagIndex) -> Result<()> {
        let path = self.index_dir()?.join("tags.json");
        index.save_to_json(&path)
    }

    /// Get the description for a domain by reading its description files.
    /// Returns None if no description file exists.
    pub fn domain_description(&self, domain_name: &str) -> Option<String> {
        let domain_path = self.root.join(domain_name);
        domains::extract_description(&domain_path)
    }

    /// List all domain folders (top-level dirs, excluding those starting with `_` or `.`).
    pub fn domains(&self) -> Result<Vec<Domain>> {
        let mut domains: Vec<Domain> = read_dir(&self.root)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();

                // Only directories
                if !path.is_dir() {
                    return None;
                }

                // Get valid UTF-8 name
                let name = path.file_name()?.to_str()?.to_string();

                // Skip excluded directories
                if is_excluded_domain(&name) {
                    return None;
                }

                let note_count = count_md_files(&path).unwrap_or(0);

                Some(Domain { name, note_count })
            })
            .collect();

        domains.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(domains)
    }

    /// Read the full content of a note by vault-relative path (e.g. "lucene/search-flow.md").
    /// Returns an error if the path does not exist.
    pub fn read_note(&self, path: &str) -> Result<String> {
        let full_path = self.root.join(path);
        if !full_path.exists() {
            bail!("note not found: {}", path);
        }
        fs::read_to_string(&full_path).with_context(|| format!("Could not read {}", path))
    }

    /// List all .md notes across the entire vault (all domains).
    pub fn all_notes(&self) -> Result<Vec<Note>> {
        let mut all = Vec::new();
        for domain in self.domains()? {
            let mut notes = self.notes_in_domain(&domain.name)?;
            all.append(&mut notes);
        }
        Ok(all)
    }

    /// List all .md notes in a named domain folder.
    pub fn notes_in_domain(&self, domain: &str) -> Result<Vec<Note>> {
        let domain_path = self.root.join(domain);
        validate_dir(&domain_path, &format!("Domain '{}'", domain))?;

        let mut notes: Vec<Note> = read_dir(&domain_path)?
            .filter_map(|entry| {
                let entry = entry.ok()?;
                let path = entry.path();

                // Only .md files
                if !path.is_file() {
                    return None;
                }
                if path.extension()?.to_str()? != "md" {
                    return None;
                }

                let filename = path.file_name()?.to_str()?.to_string();

                // Skip metadata files
                if is_metadata_file(&filename) {
                    return None;
                }
                let stem = path.file_stem()?.to_str().unwrap_or(&filename).to_string();
                let title = read_first_heading(&path).unwrap_or(stem);
                let rel_path = path.strip_prefix(&self.root).unwrap_or(&path).to_path_buf();

                Some(Note {
                    path: rel_path,
                    filename,
                    title,
                })
            })
            .collect();

        notes.sort_by(|a, b| a.filename.cmp(&b.filename));
        Ok(notes)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Validate that a path exists and is a directory.
fn validate_dir(path: &Path, label: &str) -> Result<()> {
    if !path.exists() {
        bail!("{} does not exist: {}", label, path.display());
    }
    if !path.is_dir() {
        bail!("{} is not a directory: {}", label, path.display());
    }
    Ok(())
}

/// Read directory with context about which path failed.
fn read_dir(path: &Path) -> Result<fs::ReadDir> {
    fs::read_dir(path).with_context(|| format!("Could not read directory: {}", path.display()))
}

/// Returns true if a directory name should be excluded from domains.
/// Currently excludes hidden dirs (.) and utility dirs (_).
/// TODO: Make this configurable via Config
fn is_excluded_domain(name: &str) -> bool {
    name.starts_with('.') || name.starts_with('_')
}

/// Returns true if a file is a metadata/description file and should not be counted as a note.
/// Metadata files: _description.md, description.md, or files starting with _ or .
fn is_metadata_file(filename: &str) -> bool {
    if filename.starts_with('.') || filename.starts_with('_') {
        return true;
    }
    matches!(filename, "description.md")
}

fn count_md_files(dir: &Path) -> Result<usize> {
    let mut count = 0;
    for entry in read_dir(dir)? {
        let entry = entry.context("Could not read entry")?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("md") {
            // Skip metadata files
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                if is_metadata_file(filename) {
                    continue;
                }
            }
            count += 1;
        }
    }
    Ok(count)
}

/// Read the first level-1 heading (`# Title`) from a file.
/// Only scans the first 20 lines for performance.
fn read_first_heading(path: &Path) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
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
