use crate::vault::Vault;
use anyhow::Result;

pub fn handle_read(vault: &Vault, path: String, outline: bool, line_numbers: bool) -> Result<()> {
    let content = vault.read_note(&path)?;

    if outline {
        print_outline(&content, line_numbers);
    } else {
        print_content(&content, line_numbers);
    }

    Ok(())
}

/// Calculate width needed for line numbers (minimum 6 chars)
fn line_number_width(line_count: usize) -> usize {
    format!("{}", line_count).len().max(6)
}

/// Print a line with its line number (cat -n style)
fn print_line_numbered(line_num: usize, text: impl std::fmt::Display, width: usize) {
    println!("{:>width$}\t{}", line_num, text, width = width);
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
///
/// If `line_numbers` is true, shows the original line number from the source file
/// where each heading appears (cat -n style formatting).
fn print_outline(content: &str, line_numbers: bool) {
    let lines: Vec<&str> = content.lines().collect();
    let width = if line_numbers {
        line_number_width(lines.len())
    } else {
        0
    };

    for (line_num, line) in lines.iter().enumerate() {
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
        let text = format!("{}{}", indent, line.trim());

        if line_numbers {
            print_line_numbered(line_num + 1, text, width);
        } else {
            println!("{}", text);
        }
    }
}

/// Print full content, optionally with line numbers (cat -n style).
///
/// If `line_numbers` is true, numbers all lines including blank lines,
/// right-aligned with tab separator.
fn print_content(content: &str, line_numbers: bool) {
    if line_numbers {
        let lines: Vec<&str> = content.lines().collect();
        let width = line_number_width(lines.len());

        for (i, line) in lines.iter().enumerate() {
            print_line_numbered(i + 1, line, width);
        }
    } else {
        print!("{}", content);
    }
}
