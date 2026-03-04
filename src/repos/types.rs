use std::path::PathBuf;

/// Repository information for listing.
#[derive(Debug, Clone, PartialEq)]
pub struct RepoInfo {
    /// Repository name (from filename, e.g., "datafusion" from "_repos/datafusion.md")
    pub name: String,
    
    /// Local filesystem path (if configured in vault config)
    pub local_path: Option<PathBuf>,
    
    /// Brief description (first heading or first line of content)
    pub brief: String,
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repo_info() {
        let info_without_path = RepoInfo {
            name: "test".to_string(),
            local_path: None,
            brief: "Test repo".to_string(),
        };
        
        assert_eq!(info_without_path.local_path, None);
        
        let info_with_path = RepoInfo {
            name: "test".to_string(),
            local_path: Some(PathBuf::from("/tmp/test")),
            brief: "Test repo".to_string(),
        };
        
        assert_eq!(info_with_path.local_path, Some(PathBuf::from("/tmp/test")));
    }
}
