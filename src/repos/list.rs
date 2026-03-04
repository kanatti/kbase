use anyhow::Result;

use crate::config::Config;
use crate::repos::types::RepoInfo;
use crate::repos::discover::discover_repos;
use crate::vault::Vault;

/// List all repositories in the vault, with local paths from config.
pub fn list_repos(vault: &Vault, config: &Config) -> Result<Vec<RepoInfo>> {
    let discovered = discover_repos(vault)?;
    
    let repos = discovered
        .into_iter()
        .map(|(name, brief)| {
            let local_path = config.get_repo_path(&name);
            RepoInfo {
                name,
                local_path,
                brief,
            }
        })
        .collect();
    
    Ok(repos)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{Config, VaultConfig};
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_list_repos_empty() {
        let tmp = TempDir::new().unwrap();
        
        let config = Config {
            active_vault: "test".to_string(),
            vaults: HashMap::from([
                ("test".to_string(), VaultConfig {
                    path: tmp.path().to_path_buf(),
                    repos: HashMap::new(),
                }),
            ]),
        };
        
        let vault = Vault::open(tmp.path().to_path_buf(), "test".to_string()).unwrap();
        let repos = list_repos(&vault, &config).unwrap();
        assert_eq!(repos.len(), 0);
    }

    #[test]
    fn test_list_repos_no_paths_configured() {
        let tmp = TempDir::new().unwrap();
        let repos_dir = tmp.path().join("_repos");
        fs::create_dir(&repos_dir).unwrap();
        
        fs::write(repos_dir.join("datafusion.md"), "# Apache DataFusion\n").unwrap();
        fs::write(repos_dir.join("tantivy.md"), "# Tantivy\n").unwrap();
        
        let config = Config {
            active_vault: "test".to_string(),
            vaults: HashMap::from([
                ("test".to_string(), VaultConfig {
                    path: tmp.path().to_path_buf(),
                    repos: HashMap::new(),
                }),
            ]),
        };
        
        let vault = Vault::open(tmp.path().to_path_buf(), "test".to_string()).unwrap();
        let repos = list_repos(&vault, &config).unwrap();
        
        assert_eq!(repos.len(), 2);
        assert_eq!(repos[0].name, "datafusion");
        assert_eq!(repos[0].local_path, None);
        
        assert_eq!(repos[1].name, "tantivy");
        assert_eq!(repos[1].local_path, None);
    }

    #[test]
    fn test_list_repos_with_paths() {
        let tmp = TempDir::new().unwrap();
        let repos_dir = tmp.path().join("_repos");
        fs::create_dir(&repos_dir).unwrap();
        
        fs::write(repos_dir.join("datafusion.md"), "# Apache DataFusion\n").unwrap();
        fs::write(repos_dir.join("tantivy.md"), "# Tantivy\n").unwrap();
        
        let config = Config {
            active_vault: "test".to_string(),
            vaults: HashMap::from([
                ("test".to_string(), VaultConfig {
                    path: tmp.path().to_path_buf(),
                    repos: HashMap::from([
                        ("datafusion".to_string(), PathBuf::from("/code/datafusion")),
                    ]),
                }),
            ]),
        };
        
        let vault = Vault::open(tmp.path().to_path_buf(), "test".to_string()).unwrap();
        let repos = list_repos(&vault, &config).unwrap();
        
        assert_eq!(repos.len(), 2);
        
        // datafusion has path configured
        assert_eq!(repos[0].name, "datafusion");
        assert_eq!(repos[0].local_path, Some(PathBuf::from("/code/datafusion")));
        
        // tantivy does not
        assert_eq!(repos[1].name, "tantivy");
        assert_eq!(repos[1].local_path, None);
    }
}
