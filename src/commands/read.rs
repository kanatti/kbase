use crate::vault::Vault;
use anyhow::Result;

pub fn handle_read(vault: &Vault, path: String, outline: bool) -> Result<()> {
    let content = vault.read_note(&path)?;

    if outline {
        print_outline(&content);
    } else {
        print!("{}", content);
    }

    Ok(())
}

/// Print only the heading lines from `content`, indented by heading level.
///
/// A heading line starts with 1–6 `#` characters followed by a space.
/// Indentation: `(level - 1) * 2` spaces prepended before the `#` characters.
///
/// Example output for a 3-level document:
/// ```text
/// # Title
///   ## Section
///     ### Subsection
/// ```
fn print_outline(content: &str) {
    for line in content.lines() {
        // Count leading '#' characters
        let hash_count = line.chars().take_while(|&c| c == '#').count();
        if hash_count == 0 || hash_count > 6 {
            continue;
        }
        // The character right after the hashes must be a space
        if !line[hash_count..].starts_with(' ') {
            continue;
        }
        let indent = "  ".repeat(hash_count - 1);
        println!("{}{}", indent, line.trim());
    }
}
