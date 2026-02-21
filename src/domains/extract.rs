use std::fs;
use std::path::Path;

/// Files checked for domain descriptions, in priority order.
const DESCRIPTION_FILES: &[&str] = &["_description.md", "description.md"];

/// Extract a domain description by checking description files in priority order.
/// Returns the first description found, or None if no description file exists.
pub fn extract_description(domain_path: &Path) -> Option<String> {
    for filename in DESCRIPTION_FILES {
        let path = domain_path.join(filename);
        if let Ok(content) = fs::read_to_string(&path) {
            let trimmed = content.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_string());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_extract_from_underscore_description() {
        let dir = tempdir().unwrap();
        let domain_path = dir.path();

        fs::write(
            domain_path.join("_description.md"),
            "Primary description file.",
        )
        .unwrap();

        let desc = extract_description(domain_path);
        assert_eq!(desc, Some("Primary description file.".to_string()));
    }

    #[test]
    fn test_extract_from_description() {
        let dir = tempdir().unwrap();
        let domain_path = dir.path();

        fs::write(
            domain_path.join("description.md"),
            "Standard description file.",
        )
        .unwrap();

        let desc = extract_description(domain_path);
        assert_eq!(desc, Some("Standard description file.".to_string()));
    }

    #[test]
    fn test_priority_order() {
        let dir = tempdir().unwrap();
        let domain_path = dir.path();

        // Create both files - should prefer _description.md
        fs::write(domain_path.join("description.md"), "Lower priority.").unwrap();
        fs::write(domain_path.join("_description.md"), "Higher priority.").unwrap();

        let desc = extract_description(domain_path);
        assert_eq!(desc, Some("Higher priority.".to_string()));
    }

    #[test]
    fn test_no_description_file() {
        let dir = tempdir().unwrap();
        let domain_path = dir.path();

        let desc = extract_description(domain_path);
        assert_eq!(desc, None);
    }

    #[test]
    fn test_empty_description_file() {
        let dir = tempdir().unwrap();
        let domain_path = dir.path();

        // Empty file should be treated as no description
        fs::write(domain_path.join("_description.md"), "   \n\n  ").unwrap();

        let desc = extract_description(domain_path);
        assert_eq!(desc, None);
    }

    #[test]
    fn test_multiline_description() {
        let dir = tempdir().unwrap();
        let domain_path = dir.path();

        let content = r#"# Domain Title

This is a multi-line description.
It can contain **markdown** and other formatting.

Multiple paragraphs are fine too."#;

        fs::write(domain_path.join("description.md"), content).unwrap();

        let desc = extract_description(domain_path);
        assert_eq!(desc, Some(content.to_string()));
    }
}
