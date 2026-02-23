use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::path::{Path, PathBuf};

use crate::parser::{MarkdownParser, TreeSitterParser};
use crate::vault::Vault;

use super::resolve::resolve_target;

/// Bidirectional link index for fast link queries.
/// Stores forward (source→target) and backward (target→source) mappings on disk.
pub struct LinkIndex {
    /// Forward links: source note → list of target notes
    forward: HashMap<PathBuf, Vec<PathBuf>>,
    
    /// Backward links: target note → list of source notes (backlinks)
    backward: HashMap<PathBuf, Vec<PathBuf>>,
}

impl LinkIndex {
    /// Create a LinkIndex from pre-built forward and backward maps.
    pub fn from_maps(
        mut forward: HashMap<PathBuf, Vec<PathBuf>>,
        mut backward: HashMap<PathBuf, Vec<PathBuf>>,
    ) -> Self {
        // Sort and dedup all path lists
        for paths in forward.values_mut() {
            paths.sort();
            paths.dedup();
        }
        for paths in backward.values_mut() {
            paths.sort();
            paths.dedup();
        }

        LinkIndex { forward, backward }
    }

    /// Build a LinkIndex by scanning all notes in a vault.
    pub fn build_from_vault(vault: &Vault) -> Result<(Self, usize)> {
        let all_notes = vault.all_notes()?;
        let mut parser = TreeSitterParser::new()?;

        // Build set of all note paths for resolution
        let all_note_paths: HashSet<PathBuf> = all_notes.iter().map(|n| n.path.clone()).collect();

        let mut forward: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();
        let mut backward: HashMap<PathBuf, Vec<PathBuf>> = HashMap::new();
        let mut unresolved_count = 0;

        // Single pass: parse each note, resolve links, populate both maps
        for note in &all_notes {
            let content = vault.read_note(&note.path.to_string_lossy())?;
            let parsed = parser.parse(&content)?;

            for wikilink in parsed.wikilinks {
                // Filter out non-markdown links (images, etc.)
                if !should_index_wikilink(&wikilink.target) {
                    continue;
                }

                // Resolve wikilink to actual path
                if let Some(resolved_path) = resolve_target(
                    &wikilink.target,
                    &note.path,
                    &all_note_paths,
                ) {
                    // Add to forward map: source → target
                    forward
                        .entry(note.path.clone())
                        .or_default()
                        .push(resolved_path.clone());

                    // Add to backward map: target → source
                    backward
                        .entry(resolved_path)
                        .or_default()
                        .push(note.path.clone());
                } else {
                    unresolved_count += 1;
                }
            }
        }

        Ok((LinkIndex::from_maps(forward, backward), unresolved_count))
    }

    /// Save LinkIndex to JSON files atomically.
    /// Creates two files: links-forward.json and links-backward.json
    pub fn save_to_json(&self, index_dir: &Path) -> Result<()> {
        // Create parent directory if it doesn't exist
        fs::create_dir_all(index_dir)?;

        // Convert PathBuf to String for JSON serialization
        let forward_json: HashMap<String, Vec<String>> = self
            .forward
            .iter()
            .map(|(k, v)| {
                (
                    k.to_string_lossy().to_string(),
                    v.iter().map(|p| p.to_string_lossy().to_string()).collect(),
                )
            })
            .collect();

        let backward_json: HashMap<String, Vec<String>> = self
            .backward
            .iter()
            .map(|(k, v)| {
                (
                    k.to_string_lossy().to_string(),
                    v.iter().map(|p| p.to_string_lossy().to_string()).collect(),
                )
            })
            .collect();

        // Save forward map
        save_json_atomically(&forward_json, &index_dir.join("links-forward.json"))?;

        // Save backward map
        save_json_atomically(&backward_json, &index_dir.join("links-backward.json"))?;

        Ok(())
    }

    /// Load LinkIndex from JSON files.
    #[allow(dead_code)]
    pub fn load_from_json(index_dir: &Path) -> Result<Self> {
        let forward_path = index_dir.join("links-forward.json");
        let backward_path = index_dir.join("links-backward.json");

        let forward_file = File::open(forward_path)?;
        let forward_json: HashMap<String, Vec<String>> = serde_json::from_reader(forward_file)?;

        let backward_file = File::open(backward_path)?;
        let backward_json: HashMap<String, Vec<String>> = serde_json::from_reader(backward_file)?;

        // Convert String back to PathBuf
        let forward = forward_json
            .into_iter()
            .map(|(k, v)| (PathBuf::from(k), v.into_iter().map(PathBuf::from).collect()))
            .collect();

        let backward = backward_json
            .into_iter()
            .map(|(k, v)| (PathBuf::from(k), v.into_iter().map(PathBuf::from).collect()))
            .collect();

        Ok(LinkIndex { forward, backward })
    }

    /// Get forward links for a note (notes this note links to).
    #[allow(dead_code)]
    pub fn get_forward(&self, note: &Path) -> Option<&[PathBuf]> {
        self.forward.get(note).map(|v| v.as_slice())
    }

    /// Get backward links for a note (notes that link to this note).
    #[allow(dead_code)]
    pub fn get_backward(&self, note: &Path) -> Option<&[PathBuf]> {
        self.backward.get(note).map(|v| v.as_slice())
    }
}

/// Filter wikilinks to only include markdown notes.
/// Accepts: no extension or .md extension
/// Rejects: .png, .jpg, .svg, etc.
fn should_index_wikilink(target: &str) -> bool {
    // target already has section stripped by parser
    
    // Check if it has an extension
    if let Some(ext_start) = target.rfind('.') {
        let ext = &target[ext_start + 1..];
        ext == "md" // Only accept .md
    } else {
        true // No extension = markdown note
    }
}

/// Save data to JSON file atomically (write to temp file, then rename).
fn save_json_atomically<T: serde::Serialize>(data: &T, path: &Path) -> Result<()> {
    let temp_path = path.with_extension("tmp");
    let file = File::create(&temp_path)?;
    serde_json::to_writer_pretty(file, data)?;
    fs::rename(temp_path, path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_index_wikilink() {
        // Bare names (no extension) - accept
        assert!(should_index_wikilink("note"));
        assert!(should_index_wikilink("domain/note"));
        
        // Explicit .md - accept
        assert!(should_index_wikilink("note.md"));
        assert!(should_index_wikilink("domain/note.md"));
        
        // Images and other extensions - reject
        assert!(!should_index_wikilink("image.png"));
        assert!(!should_index_wikilink("diagram.svg"));
        assert!(!should_index_wikilink("photo.jpg"));
        assert!(!should_index_wikilink("folder/screenshot.png"));
    }

    #[test]
    fn test_from_maps_sorts_and_dedups() {
        let mut forward = HashMap::new();
        forward.insert(
            PathBuf::from("a.md"),
            vec![
                PathBuf::from("c.md"),
                PathBuf::from("b.md"),
                PathBuf::from("c.md"), // duplicate
            ],
        );

        let backward = HashMap::new();

        let index = LinkIndex::from_maps(forward, backward);

        // Should be sorted and deduped
        assert_eq!(
            index.forward.get(&PathBuf::from("a.md")).unwrap(),
            &vec![PathBuf::from("b.md"), PathBuf::from("c.md")]
        );
    }
}
