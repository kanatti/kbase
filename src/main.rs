mod config;
mod vault;

use anyhow::Result;
use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(name = "kb", about = "Knowledge base CLI for markdown vaults", version)]
struct Cli {
    /// Override vault path (env: KB_VAULT)
    #[arg(long, global = true, env = "KB_VAULT")]
    vault: Option<String>,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Show or set config
    Config {
        #[command(subcommand)]
        action: Option<ConfigAction>,
    },

    /// List all domains with note counts
    Domains {
        /// Sort by: name (default) or count
        #[arg(long, default_value = "name")]
        sort: String,
    },

    /// List notes (all, or filtered by domain or search term)
    Notes {
        /// Show only notes in this domain
        #[arg(long)]
        domain: Option<String>,

        /// Filter by name/title match (not yet implemented)
        #[arg(long)]
        term: Option<String>,

        /// Show filenames only, no titles
        #[arg(long)]
        files: bool,
    },

    /// Print a note's content (raw markdown or heading outline)
    Read {
        /// Vault-relative path with .md extension (e.g. lucene/search-flow.md)
        path: String,

        /// Print heading outline only, indented by level
        #[arg(long)]
        outline: bool,
    },
}

#[derive(Subcommand)]
enum ConfigAction {
    /// Set a config value (e.g. `kb config set vault /path`)
    Set { key: String, value: String },
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Config { action } => match action {
            None => config::show()?,
            Some(ConfigAction::Set { key, value }) => config::set(&key, &value)?,
        },

        Command::Domains { sort } => {
            let vault = open_vault(cli.vault.as_deref())?;
            let mut domains = vault.domains()?;

            if sort == "count" {
                domains.sort_by(|a, b| b.note_count.cmp(&a.note_count));
            }

            if domains.is_empty() {
                println!("No domains found in vault.");
                return Ok(());
            }

            let max_name = domains.iter().map(|d| d.name.len()).max().unwrap_or(0);
            for d in &domains {
                let n = d.note_count;
                let label = if n == 1 { "note" } else { "notes" };
                println!("{:<width$}  {} {}", d.name, n, label, width = max_name);
            }
        }

        Command::Notes { domain, term, files } => {
            if term.is_some() {
                eprintln!("--term search is not yet implemented");
                std::process::exit(1);
            }

            let vault = open_vault(cli.vault.as_deref())?;

            let notes = match &domain {
                Some(d) => vault.notes_in_domain(d)?,
                None => vault.all_notes()?,
            };

            if notes.is_empty() {
                println!("No notes found.");
                return Ok(());
            }

            if files {
                for note in &notes {
                    println!("{}", note.path.display());
                }
            } else {
                let max_name = notes
                    .iter()
                    .map(|n| n.path.display().to_string().len())
                    .max()
                    .unwrap_or(0);

                for note in &notes {
                    println!(
                        "{:<width$}  {}",
                        note.path.display(),
                        note.title,
                        width = max_name
                    );
                }
            }
        }

        Command::Read { path, outline } => {
            let vault = open_vault(cli.vault.as_deref())?;
            let content = vault.read_note(&path)?;

            if outline {
                print_outline(&content);
            } else {
                print!("{}", content);
            }
        }
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

/// Resolve vault path: --vault / KB_VAULT env (via clap) → config file.
fn open_vault(vault_override: Option<&str>) -> Result<vault::Vault> {
    let path = match vault_override {
        Some(v) => std::path::PathBuf::from(shellexpand::tilde(v).as_ref()),
        None => config::Config::load()?.vault,
    };
    vault::Vault::open(path)
}
