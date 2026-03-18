use crate::vault::Vault;
use crate::search::domain::{IndexedNote};
use crate::search::schema::SearchSchema;
use anyhow::Result;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use tantivy::{Index, doc};

/// Service for building search indexes from vaults
pub struct SearchIndexer {
    schema: SearchSchema,
}

impl SearchIndexer {
    pub fn new() -> Self {
        SearchIndexer {
            schema: SearchSchema::new(),
        }
    }
    
    /// Build a search index from all notes in a vault
    /// 
    /// Returns statistics about the indexing operation
    pub fn build_from_vault(&self, vault: &Vault) -> Result<IndexStats> {
        let start = Instant::now();
        let index_path = vault.index_dir()?.join("search.tantivy");
        
        // Prepare index directory
        self.prepare_index_directory(&index_path)?;
        
        // Create Tantivy index
        let index = Index::create_in_dir(&index_path, self.schema.inner().clone())?;
        let mut writer = index.writer(50_000_000)?;
        
        // Transform and index all notes
        let notes = vault.all_notes()?;
        let mut stats = IndexStats::new(index_path.clone());
        
        for note in &notes {
            match IndexedNote::from_vault_note(note, vault) {
                Ok(indexed_note) => {
                    let doc = self.create_document(&indexed_note);
                    writer.add_document(doc)?;
                    stats.increment_indexed();
                }
                Err(e) => {
                    eprintln!("Warning: Failed to index {}: {}", note.path.display(), e);
                    stats.increment_failed();
                }
            }
        }
        
        // Commit atomically
        writer.commit()?;
        
        stats.set_elapsed(start.elapsed());
        Ok(stats)
    }
    
    fn prepare_index_directory(&self, path: &Path) -> Result<()> {
        use std::fs;
        
        if path.exists() {
            fs::remove_dir_all(path)?;
        }
        fs::create_dir_all(path)?;
        Ok(())
    }
    
    fn create_document(&self, note: &IndexedNote) -> tantivy::TantivyDocument {
        let fields = self.schema.fields();
        
        doc!(
            fields.path => note.path().to_string_lossy().to_string(),
            fields.domain => note.domain(),
            fields.title => note.title(),
            fields.content => note.content(),
        )
    }
}

/// Statistics from an indexing operation
#[derive(Debug, Clone)]
pub struct IndexStats {
    index_path: PathBuf,
    documents_indexed: usize,
    documents_failed: usize,
    elapsed: Duration,
}

impl IndexStats {
    fn new(index_path: PathBuf) -> Self {
        IndexStats {
            index_path,
            documents_indexed: 0,
            documents_failed: 0,
            elapsed: Duration::default(),
        }
    }
    
    fn increment_indexed(&mut self) {
        self.documents_indexed += 1;
    }
    
    fn increment_failed(&mut self) {
        self.documents_failed += 1;
    }
    
    fn set_elapsed(&mut self, elapsed: Duration) {
        self.elapsed = elapsed;
    }
    
    pub fn indexed_count(&self) -> usize { self.documents_indexed }
    pub fn failed_count(&self) -> usize { self.documents_failed }
    pub fn index_path(&self) -> &Path { &self.index_path }
    pub fn elapsed(&self) -> Duration { self.elapsed }
}
