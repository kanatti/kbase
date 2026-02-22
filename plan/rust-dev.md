# Rust Development Best Practices for kbase

## Import/Use Statement Mistakes

### ❌ Don't import inside functions
```rust
fn count_md_files(dir: &Path) -> Result<usize> {
    use walkdir::WalkDir;  // ← WRONG: imports belong at top of file
    
    for entry in WalkDir::new(dir) {
        // ...
    }
}
```

### ✅ Import at the top of the file
```rust
// At the top of src/vault.rs
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;  // ← RIGHT: with other imports

// ... rest of file

fn count_md_files(dir: &Path) -> Result<usize> {
    for entry in WalkDir::new(dir) {
        // ...
    }
}
```

**Why:** 
- Easier to see all dependencies at a glance
- Standard Rust convention
- Better organization and maintainability

---

## TODO: Add more best practices as we encounter them
