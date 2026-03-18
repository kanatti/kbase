use std::path::{Path, PathBuf};
use anyhow::{Result, bail, Context};
use tantivy::Index;

/// Repository for managing search index persistence
pub struct SearchRepository {
    index_path: PathBuf,
}

impl SearchRepository {
    pub fn new(index_path: PathBuf) -> Self {
        SearchRepository { index_path }
    }
    
    /// Check if index exists on disk
    pub fn exists(&self) -> bool {
        self.index_path.join("meta.json").exists()
    }
    
    /// Open the index for reading
    /// 
    /// Returns error if index doesn't exist or is corrupted
    pub fn open(&self) -> Result<Index> {
        if !self.exists() {
            bail!(
                "Search index not found at {}\n\
                 Run 'kbase index' to build the search index.",
                self.index_path.display()
            );
        }
        
        Index::open_in_dir(&self.index_path)
            .with_context(|| {
                format!(
                    "Failed to open search index at {}\n\
                     The index may be corrupted. Try rebuilding with 'kbase index'.",
                    self.index_path.display()
                )
            })
    }
    
    pub fn path(&self) -> &Path {
        &self.index_path
    }
}
