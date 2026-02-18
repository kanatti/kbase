// Public interface for tag functionality

pub mod extract;
pub mod index;

// Re-export commonly used types and functions
pub use extract::extract_tags;
pub use index::TagIndex;