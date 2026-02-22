use kbase::parser::{MarkdownParser, tree_sitter::TreeSitterParser};
use std::fs;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run --example parse_demo <file.md>");
        std::process::exit(1);
    }

    let file_path = Path::new(&args[1]);
    let content = fs::read_to_string(file_path)?;

    println!("Parsing: {}", file_path.display());
    println!("{}", "─".repeat(60));

    let mut parser = TreeSitterParser::new()?;
    let parsed = parser.parse(&content)?;

    // Display title (or use filename if no H1)
    let title = if parsed.title.is_empty() {
        file_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Untitled")
    } else {
        &parsed.title
    };
    println!("Title: {}", title);

    println!("\nHeadings ({})", parsed.headings.len());
    for h in &parsed.headings {
        println!("  {} L{}: {}", "#".repeat(h.level as usize), h.line, h.text);
    }

    println!("\nWikilinks ({})", parsed.wikilinks.len());
    for link in &parsed.wikilinks {
        let mut link_str = format!("[[{}]]", link.target);
        if let Some(alias) = &link.alias {
            link_str = format!("[[{}|{}]]", link.target, alias);
        }
        if let Some(section) = &link.section {
            link_str = format!("  {} (→ section: {})", link_str, section);
        } else {
            link_str = format!("  {}", link_str);
        }
        println!("{} at L{}:{}", link_str, link.line, link.column);
    }

    println!("\nTags ({})", parsed.tags.len());
    for tag in &parsed.tags {
        println!("  #{}", tag);
    }

    println!("\nBody length: {} bytes", parsed.body.len());

    Ok(())
}
