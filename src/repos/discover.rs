use anyhow::{Context, Result};
use std::fs;

use crate::vault::Vault;

/// Discover all repository description files in the _repos directory.
/// Returns a list of (name, brief_description) tuples.
pub fn discover_repos(vault: &Vault) -> Result<Vec<(String, String)>> {
    let repos_dir = vault.repos_dir();
    
    if !repos_dir.exists() {
        return Ok(Vec::new());
    }
    
    let mut repos = Vec::new();
    
    for entry in fs::read_dir(&repos_dir)
        .with_context(|| format!("Failed to read _repos directory: {}", repos_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        
        // Only consider .md files
        if !path.is_file() || path.extension().and_then(|s| s.to_str()) != Some("md") {
            continue;
        }
        
        // Extract name from filename (e.g., "datafusion.md" -> "datafusion")
        let name = match path.file_stem().and_then(|s| s.to_str()) {
            Some(n) => n.to_string(),
            None => continue,
        };
        
        // Read brief description (first heading)
        let brief = extract_brief(&path).unwrap_or_else(|| name.clone());
        
        repos.push((name, brief));
    }
    
    // Sort by name for consistent output
    repos.sort_by(|a, b| a.0.cmp(&b.0));
    
    Ok(repos)
}

/// Extract a brief description from a repo file.
/// Returns the first # heading, or the first non-empty line if no heading found.
fn extract_brief(path: &std::path::Path) -> Option<String> {
    let content = fs::read_to_string(path).ok()?;
    
    for line in content.lines().take(20) {
        let trimmed = line.trim();
        
        // First heading wins
        if let Some(heading) = trimmed.strip_prefix("# ") {
            let text = heading.trim();
            if !text.is_empty() {
                return Some(text.to_string());
            }
        }
        
        // Fall back to first non-empty line
        if !trimmed.is_empty() && !trimmed.starts_with('#') {
            return Some(trimmed.to_string());
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_discover_repos_empty() {
        let tmp = TempDir::new().unwrap();
        let vault = Vault::open(tmp.path().to_path_buf(), "test".to_string()).unwrap();
        let repos = discover_repos(&vault).unwrap();
        assert_eq!(repos.len(), 0);
    }

    #[test]
    fn test_discover_repos() {
        let tmp = TempDir::new().unwrap();
        let repos_dir = tmp.path().join("_repos");
        fs::create_dir(&repos_dir).unwrap();
        
        fs::write(repos_dir.join("datafusion.md"), "# Apache DataFusion\n\nQuery engine").unwrap();
        fs::write(repos_dir.join("tantivy.md"), "# Tantivy\n\nSearch library").unwrap();
        
        let vault = Vault::open(tmp.path().to_path_buf(), "test".to_string()).unwrap();
        let repos = discover_repos(&vault).unwrap();
        
        assert_eq!(repos.len(), 2);
        assert_eq!(repos[0].0, "datafusion");
        assert_eq!(repos[0].1, "Apache DataFusion");
        assert_eq!(repos[1].0, "tantivy");
        assert_eq!(repos[1].1, "Tantivy");
    }

    #[test]
    fn test_discover_repos_ignores_non_md() {
        let tmp = TempDir::new().unwrap();
        let repos_dir = tmp.path().join("_repos");
        fs::create_dir(&repos_dir).unwrap();
        
        fs::write(repos_dir.join("datafusion.md"), "# DataFusion").unwrap();
        fs::write(repos_dir.join("README.txt"), "Not markdown").unwrap();
        fs::create_dir(repos_dir.join("subdir")).unwrap();
        
        let vault = Vault::open(tmp.path().to_path_buf(), "test".to_string()).unwrap();
        let repos = discover_repos(&vault).unwrap();
        
        assert_eq!(repos.len(), 1);
        assert_eq!(repos[0].0, "datafusion");
    }

    #[test]
    fn test_extract_brief_from_heading() {
        let tmp = TempDir::new().unwrap();
        let file = tmp.path().join("test.md");
        fs::write(&file, "# My Repo\n\nSome content").unwrap();
        
        let brief = extract_brief(&file).unwrap();
        assert_eq!(brief, "My Repo");
    }

    #[test]
    fn test_extract_brief_no_heading() {
        let tmp = TempDir::new().unwrap();
        let file = tmp.path().join("test.md");
        fs::write(&file, "First line of content\n\nSecond line").unwrap();
        
        let brief = extract_brief(&file).unwrap();
        assert_eq!(brief, "First line of content");
    }
}
