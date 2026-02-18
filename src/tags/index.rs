use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

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

    /// Load TagIndex from JSON file.
    /// Returns empty index if file doesn't exist.
    pub fn load_from_json(json_path: &Path) -> Result<Self> {
        let by_tag = if json_path.exists() {
            let file = std::fs::File::open(json_path)?;
            serde_json::from_reader(file)?
        } else {
            HashMap::new()
        };

        Ok(Self::from_tag_map(by_tag))
    }

    /// Save TagIndex to JSON file atomically.
    /// Writes to temp file first, then renames to avoid partial writes.
    pub fn save_to_json(&self, json_path: &Path) -> Result<()> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = json_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Atomic write: temp file -> rename
        // json_path: ~/.kb/indexes/vault-name/tags.json (final destination)
        // temp_path: ~/.kb/indexes/vault-name/tags.json.tmp (temporary write target)
        let temp_path = json_path.with_extension("json.tmp");
        let file = std::fs::File::create(&temp_path)?;
        serde_json::to_writer_pretty(file, &self.by_tag)?;
        std::fs::rename(temp_path, json_path)?;

        Ok(())
    }

    /// Get all tags with their note counts, sorted alphabetically by tag name.
    pub fn all_tags(&self) -> Vec<(&String, usize)> {
        let mut tags: Vec<_> = self
            .by_tag
            .iter()
            .map(|(tag, paths)| (tag, paths.len()))
            .collect();
        tags.sort_by_key(|(tag, _)| tag.as_str());
        tags
    }

    /// Get all tags with their note counts, sorted by count (descending).
    pub fn all_tags_by_count(&self) -> Vec<(&String, usize)> {
        let mut tags: Vec<_> = self
            .by_tag
            .iter()
            .map(|(tag, paths)| (tag, paths.len()))
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use tempfile::tempdir;

    fn create_test_tag_map() -> HashMap<String, Vec<String>> {
        let mut tags = HashMap::new();
        tags.insert(
            "rust".to_string(),
            vec!["rust/basics.md".to_string(), "rust/advanced.md".to_string()],
        );
        tags.insert(
            "wip".to_string(),
            vec!["rust/basics.md".to_string(), "lucene/study.md".to_string()],
        );
        tags.insert(
            "deep-dive".to_string(),
            vec!["lucene/internals.md".to_string()],
        );
        tags.insert("home".to_string(), vec!["01-home.md".to_string()]); // Root file
        tags
    }

    #[test]
    fn test_from_tag_map_builds_index() {
        let tag_map = create_test_tag_map();
        let index = TagIndex::from_tag_map(tag_map);

        // Test forward lookup
        assert_eq!(
            index.notes_with_tag("rust"),
            vec!["rust/advanced.md", "rust/basics.md"]
        );
        assert_eq!(
            index.notes_with_tag("wip"),
            vec!["lucene/study.md", "rust/basics.md"]
        );
        assert_eq!(index.notes_with_tag("nonexistent"), Vec::<String>::new());
    }

    #[test]
    fn test_all_tags_sorting() {
        let tag_map = create_test_tag_map();
        let index = TagIndex::from_tag_map(tag_map);

        // By name (alphabetical)
        let by_name = index.all_tags();
        assert_eq!(
            by_name,
            vec![
                (&"deep-dive".to_string(), 1),
                (&"home".to_string(), 1),
                (&"rust".to_string(), 2),
                (&"wip".to_string(), 2)
            ]
        );

        // By count (descending) - for same counts, order is not guaranteed
        let by_count = index.all_tags_by_count();
        assert_eq!(by_count.len(), 4);

        // First two should be tags with count 2
        assert!(by_count[0].1 == 2);
        assert!(by_count[1].1 == 2);
        // Last two should be tags with count 1
        assert!(by_count[2].1 == 1);
        assert!(by_count[3].1 == 1);
    }

    #[test]
    fn test_filter_by_domains() {
        let tag_map = create_test_tag_map();
        let index = TagIndex::from_tag_map(tag_map);

        // Filter to rust domain only
        let rust_only = index.filter_by_domains(&["rust".to_string()]);
        assert_eq!(rust_only.len(), 2); // rust, wip tags remain
        assert_eq!(
            rust_only.get("rust").unwrap(),
            &vec!["rust/advanced.md", "rust/basics.md"]
        );
        assert_eq!(rust_only.get("wip").unwrap(), &vec!["rust/basics.md"]);
        assert!(rust_only.get("deep-dive").is_none()); // Filtered out (lucene domain)
        assert!(rust_only.get("home").is_none()); // Filtered out (root file, no domain)

        // Filter to multiple domains
        let multi = index.filter_by_domains(&["rust".to_string(), "lucene".to_string()]);
        assert_eq!(multi.len(), 3); // rust, wip, deep-dive remain, home filtered out
        assert!(multi.get("home").is_none()); // Root files never match domain filters

        // Filter to nonexistent domain
        let empty = index.filter_by_domains(&["nonexistent".to_string()]);
        assert!(empty.is_empty());
    }

    #[test]
    fn test_save_and_load_json() -> Result<()> {
        let tag_map = create_test_tag_map();
        let original = TagIndex::from_tag_map(tag_map);

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
    fn test_load_from_nonexistent_file() -> Result<()> {
        let temp_dir = tempdir()?;
        let json_path = temp_dir.path().join("nonexistent.json");

        let index = TagIndex::load_from_json(&json_path)?;
        assert!(index.all_tags().is_empty());
        assert!(index.notes_with_tag("any").is_empty());

        Ok(())
    }

    #[test]
    fn test_empty_index() {
        let index = TagIndex::from_tag_map(HashMap::new());
        assert!(index.all_tags().is_empty());
        assert!(index.all_tags_by_count().is_empty());
        assert!(index.notes_with_tag("any").is_empty());
        assert!(index.filter_by_domains(&["any".to_string()]).is_empty());
    }
}
