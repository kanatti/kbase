use crate::parser::{MarkdownParser, TreeSitterParser};
use crate::vault::Vault;
use anyhow::Result;

pub fn handle_read(vault: &Vault, path: String, outline: bool, line_numbers: bool) -> Result<()> {
    let content = vault.read_note(&path)?;

    if outline {
        print_outline(&content, line_numbers)?;
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

/// Print heading outline, indented by level.
/// Optionally shows line numbers where each heading appears.
fn print_outline(content: &str, line_numbers: bool) -> Result<()> {
    let mut parser = TreeSitterParser::new()?;
    let parsed = parser.parse(content)?;

    let width = if line_numbers {
        line_number_width(content.lines().count())
    } else {
        0
    };

    // Print each heading with indentation and markdown markers
    for heading in parsed.headings {
        let indent = "  ".repeat((heading.level - 1) as usize);
        let markers = "#".repeat(heading.level as usize);
        let text = format!("{}{} {}", indent, markers, heading.text);

        if line_numbers {
            print_line_numbered(heading.line, text, width);
        } else {
            println!("{}", text);
        }
    }

    Ok(())
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
