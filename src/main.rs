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

    /// List all topics with note counts
    Topics {
        /// Sort by: name (default) or count
        #[arg(long, default_value = "name")]
        sort: String,
    },

    /// List notes (all, or filtered by topic or search term)
    Notes {
        /// Show only notes in this topic
        #[arg(long)]
        topic: Option<String>,

        /// Filter by name/title match (not yet implemented)
        #[arg(long)]
        term: Option<String>,

        /// Show filenames only, no titles
        #[arg(long)]
        files: bool,
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

        Command::Topics { sort } => {
            let vault = open_vault(cli.vault.as_deref())?;
            let mut topics = vault.topics()?;

            if sort == "count" {
                topics.sort_by(|a, b| b.note_count.cmp(&a.note_count));
            }

            if topics.is_empty() {
                println!("No topics found in vault.");
                return Ok(());
            }

            let max_name = topics.iter().map(|t| t.name.len()).max().unwrap_or(0);
            for t in &topics {
                let n = t.note_count;
                let label = if n == 1 { "note" } else { "notes" };
                println!("{:<width$}  {} {}", t.name, n, label, width = max_name);
            }
        }

        Command::Notes { topic, term, files } => {
            if term.is_some() {
                eprintln!("--term search is not yet implemented");
                std::process::exit(1);
            }

            let vault = open_vault(cli.vault.as_deref())?;

            let notes = match &topic {
                Some(t) => vault.notes_in_topic(t)?,
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
    }

    Ok(())
}


/// Resolve vault path: --vault / KB_VAULT env (via clap) → config file.
fn open_vault(vault_override: Option<&str>) -> Result<vault::Vault> {
    let path = match vault_override {
        Some(v) => std::path::PathBuf::from(shellexpand::tilde(v).as_ref()),
        None => config::Config::load()?.vault,
    };
    vault::Vault::open(path)
}
