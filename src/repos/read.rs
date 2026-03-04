use anyhow::{bail, Context, Result};
use std::fs;

use crate::vault::Vault;

/// Read the full content of a repository description file.
pub fn read_repo_description(vault: &Vault, repo_name: &str) -> Result<String> {
    let repo_path = vault.repos_dir().join(format!("{}.md", repo_name));
    
    if !repo_path.exists() {
        bail!("Repository '{}' not found in vault", repo_name);
    }
    
    fs::read_to_string(&repo_path)
        .with_context(|| format!("Failed to read repository description: {}", repo_path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_read_repo_description() {
        let tmp = TempDir::new().unwrap();
        let repos_dir = tmp.path().join("_repos");
        fs::create_dir(&repos_dir).unwrap();
        
        let content = "# Apache DataFusion\n\nQuery engine in Rust.\n";
        fs::write(repos_dir.join("datafusion.md"), content).unwrap();
        
        let vault = Vault::open(tmp.path().to_path_buf(), "test".to_string()).unwrap();
        let result = read_repo_description(&vault, "datafusion").unwrap();
        assert_eq!(result, content);
    }

    #[test]
    fn test_read_repo_description_not_found() {
        let tmp = TempDir::new().unwrap();
        
        let vault = Vault::open(tmp.path().to_path_buf(), "test".to_string()).unwrap();
        let result = read_repo_description(&vault, "nonexistent");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("not found"));
    }
}
