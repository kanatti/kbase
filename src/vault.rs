use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

pub struct Vault {
    pub root: PathBuf,
    pub name: String,
}

pub struct Domain {
    pub name: String,
    pub path: PathBuf,
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
        if !root.exists() {
            bail!("Vault path does not exist: {}", root.display());
        }
        if !root.is_dir() {
            bail!("Vault path is not a directory: {}", root.display());
        }
        Ok(Vault { root, name })
    }

    /// List all domain folders (top-level dirs, excluding those starting with `_` or `.`).
    pub fn domains(&self) -> Result<Vec<Domain>> {
        let mut domains = Vec::new();

        for entry in fs::read_dir(&self.root).context("Could not read vault directory")? {
            let entry = entry.context("Could not read directory entry")?;
            let path = entry.path();

            if !path.is_dir() {
                continue;
            }

            let name = match path.file_name().and_then(|n| n.to_str()) {
                Some(n) => n.to_string(),
                None => continue,
            };

            // Exclude hidden dirs and any dir starting with _ (covers _logs, __templates, etc.)
            if name.starts_with('.') || name.starts_with('_') {
                continue;
            }

            let note_count = count_md_files(&path).unwrap_or(0);

            domains.push(Domain {
                name,
                path,
                note_count,
            });
        }

        // Default: alphabetical
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

        if !domain_path.exists() {
            bail!("Domain not found: '{}'", domain);
        }
        if !domain_path.is_dir() {
            bail!("'{}' is not a domain folder", domain);
        }

        let mut notes = Vec::new();

        for entry in fs::read_dir(&domain_path).context("Could not read domain directory")? {
            let entry = entry.context("Could not read entry")?;
            let path = entry.path();

            if !path.is_file() {
                continue;
            }
            if path.extension().and_then(|e| e.to_str()) != Some("md") {
                continue;
            }

            let filename = match path.file_name().and_then(|n| n.to_str()) {
                Some(n) => n.to_string(),
                None => continue,
            };

            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or(&filename)
                .to_string();

            let title = read_first_heading(&path).unwrap_or(stem);

            let rel_path = path.strip_prefix(&self.root).unwrap_or(&path).to_path_buf();

            notes.push(Note {
                path: rel_path,
                filename,
                title,
            });
        }

        notes.sort_by(|a, b| a.filename.cmp(&b.filename));

        Ok(notes)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn count_md_files(dir: &Path) -> Result<usize> {
    let mut count = 0;
    for entry in fs::read_dir(dir).context("Could not read directory")? {
        let entry = entry.context("Could not read entry")?;
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("md") {
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
