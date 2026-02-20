use anyhow::Result;
use std::collections::HashMap;
use std::fs::{self, File};
use std::path::Path;

use crate::tags::extract_tags;
use crate::vault::Vault;

/// Bidirectional index for fast tag queries.
/// Stores tag->paths mapping on disk.
pub struct TagIndex {
    /// Primary storage: tag name -> list of note paths
    by_tag: HashMap<String, Vec<String>>,
}

impl TagIndex {
    /// Build TagIndex from a tag->paths mapping.
    pub fn from_tag_map(mut by_tag: HashMap<String, Vec<String>>) -> Self {
        // Sort paths within each tag for consistent ordering
        for paths in by_tag.values_mut() {
            paths.sort();
        }

        TagIndex { by_tag }
    }

    /// Create a new builder for constructing a TagIndex.
    #[allow(dead_code)]
    pub fn builder() -> TagIndexBuilder {
        TagIndexBuilder::new()
    }

    /// Build a TagIndex by scanning all notes in a vault.
    pub fn build_from_vault(vault: &Vault) -> Result<Self> {
        let mut tag_map: HashMap<String, Vec<String>> = HashMap::new();
        let all_notes = vault.all_notes()?;

        for note in &all_notes {
            let path_str = note.path.to_string_lossy().to_string();
            let content = vault.read_note(&path_str)?;
            let tags = extract_tags(&content);

            for tag in tags {
                tag_map
                    .entry(tag)
                    .or_insert_with(Vec::new)
                    .push(path_str.clone());
            }
        }

        Ok(TagIndex::from_tag_map(tag_map))
    }

    /// Load TagIndex from JSON file.
    pub fn load_from_json(json_path: &Path) -> Result<Self> {
        let file = File::open(json_path)?;
        let by_tag = serde_json::from_reader(file)?;
        Ok(Self::from_tag_map(by_tag))
    }

    /// Save TagIndex to JSON file atomically.
    /// Writes to temp file first, then renames to avoid partial writes.
    pub fn save_to_json(&self, json_path: &Path) -> Result<()> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = json_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Atomic write: temp file -> rename
        // json_path: ~/.kb/indexes/vault-name/tags.json (final destination)
        // temp_path: ~/.kb/indexes/vault-name/tags.json.tmp (temporary write target)
        let temp_path = json_path.with_extension("json.tmp");
        let file = File::create(&temp_path)?;
        serde_json::to_writer_pretty(file, &self.by_tag)?;
        fs::rename(temp_path, json_path)?;

        Ok(())
    }

    /// Get all tags with their note counts, sorted alphabetically by tag name.
    pub fn all_tags(&self) -> Vec<(String, usize)> {
        let mut tags: Vec<_> = self
            .by_tag
            .iter()
            .map(|(tag, paths)| (tag.clone(), paths.len()))
            .collect();
        tags.sort_by(|(a, _), (b, _)| a.cmp(b));
        tags
    }

    /// Get all tags with their note counts, sorted by count (descending).
    pub fn all_tags_by_count(&self) -> Vec<(String, usize)> {
        let mut tags: Vec<_> = self
            .by_tag
            .iter()
            .map(|(tag, paths)| (tag.clone(), paths.len()))
            .collect();
        tags.sort_by(|(_, a), (_, b)| b.cmp(a)); // Descending count
        tags
    }

    /// Get all note paths that have the specified tag.
    pub fn notes_with_tag(&self, tag: &str) -> Vec<String> {
        self.by_tag.get(tag).cloned().unwrap_or_default()
    }

    /// Filter tags to only include notes in the specified domains.
    /// Returns a new tag->paths mapping with domain filtering applied.
    /// TODO: Use this in `kb tags --domain <domain>` command
    #[allow(dead_code)]
    pub fn filter_by_domains(&self, domains: &[String]) -> HashMap<String, Vec<String>> {
        let mut filtered = HashMap::new();

        for (tag, paths) in &self.by_tag {
            let domain_paths: Vec<String> = paths
                .iter()
                .filter(|path| {
                    // Extract domain: everything before first '/'
                    if let Some(slash_pos) = path.find('/') {
                        let domain = &path[..slash_pos];
                        domains.contains(&domain.to_string())
                    } else {
                        // Root files (no '/') have no domain, never match
                        false
                    }
                })
                .cloned()
                .collect();

            if !domain_paths.is_empty() {
                filtered.insert(tag.clone(), domain_paths);
            }
        }

        filtered
    }
}

/// Builder for constructing a TagIndex incrementally.
#[allow(dead_code)]
pub struct TagIndexBuilder {
    map: HashMap<String, Vec<String>>,
}

#[allow(dead_code)]
impl TagIndexBuilder {
    /// Create a new empty builder.
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    /// Add a tag-path association.
    pub fn add(&mut self, tag: &str, path: &str) -> &mut Self {
        self.map
            .entry(tag.to_string())
            .or_insert_with(Vec::new)
            .push(path.to_string());
        self
    }

    /// Build the final TagIndex.
    pub fn build(self) -> TagIndex {
        TagIndex::from_tag_map(self.map)
    }
}

impl Default for TagIndexBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    fn create_test_index() -> TagIndex {
        let mut builder = TagIndex::builder();
        builder.add("rust", "rust/basics.md");
        builder.add("rust", "rust/advanced.md");
        builder.add("rust", "rust/ownership.md");
        builder.add("wip", "rust/basics.md");
        builder.add("wip", "lucene/study.md");
        builder.add("deep-dive", "lucene/internals.md");
        builder.build()
    }

    #[test]
    fn test_from_tag_map_builds_index() {
        let index = create_test_index();

        // Test forward lookup
        assert_eq!(
            index.notes_with_tag("rust"),
            vec!["rust/advanced.md", "rust/basics.md", "rust/ownership.md"]
        );
        assert_eq!(
            index.notes_with_tag("wip"),
            vec!["lucene/study.md", "rust/basics.md"]
        );
        assert_eq!(index.notes_with_tag("nonexistent"), Vec::<String>::new());
    }

    #[test]
    fn test_all_tags_sorted_by_name() {
        let index = create_test_index();
        let by_name = index.all_tags();

        assert_eq!(
            by_name,
            vec![
                ("deep-dive".into(), 1),
                ("rust".into(), 3),
                ("wip".into(), 2)
            ]
        );
    }

    #[test]
    fn test_all_tags_sorted_by_count() {
        let index = create_test_index();
        let by_count = index.all_tags_by_count();

        assert_eq!(
            by_count,
            vec![
                ("rust".into(), 3),
                ("wip".into(), 2),
                ("deep-dive".into(), 1)
            ]
        );
    }

    #[test]
    fn test_filter_by_single_domain() {
        let index = create_test_index();
        let filtered = index.filter_by_domains(&["rust".to_string()]);

        // Only rust and wip tags should remain (wip has rust/basics.md)
        assert_eq!(filtered.len(), 2);
        assert_eq!(
            filtered.get("rust").unwrap(),
            &vec!["rust/advanced.md", "rust/basics.md", "rust/ownership.md"]
        );
        assert_eq!(filtered.get("wip").unwrap(), &vec!["rust/basics.md"]);
        assert!(filtered.get("deep-dive").is_none());
    }

    #[test]
    fn test_filter_by_multiple_domains() {
        let index = create_test_index();
        let filtered = index.filter_by_domains(&["rust".to_string(), "lucene".to_string()]);

        // All three tags should remain
        assert_eq!(filtered.len(), 3);
        assert!(filtered.contains_key("rust"));
        assert!(filtered.contains_key("wip"));
        assert!(filtered.contains_key("deep-dive"));
    }

    #[test]
    fn test_filter_by_nonexistent_domain() {
        let index = create_test_index();
        let filtered = index.filter_by_domains(&["nonexistent".to_string()]);

        assert!(filtered.is_empty());
    }

    #[test]
    fn test_save_and_load_json() -> Result<()> {
        let original = create_test_index();

        // Save to temporary file
        let temp_dir = tempdir()?;
        let json_path = temp_dir.path().join("test_tags.json");
        original.save_to_json(&json_path)?;

        // Load back and verify
        let loaded = TagIndex::load_from_json(&json_path)?;
        assert_eq!(loaded.all_tags(), original.all_tags());
        assert_eq!(
            loaded.notes_with_tag("rust"),
            original.notes_with_tag("rust")
        );

        Ok(())
    }

    #[test]
    fn test_load_from_nonexistent_file() {
        let temp_dir = tempdir().unwrap();
        let json_path = temp_dir.path().join("nonexistent.json");

        let result = TagIndex::load_from_json(&json_path);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_index() {
        let index = TagIndex::builder().build();
        assert!(index.all_tags().is_empty());
        assert!(index.all_tags_by_count().is_empty());
        assert!(index.notes_with_tag("any").is_empty());
        assert!(index.filter_by_domains(&["any".to_string()]).is_empty());
    }
}
