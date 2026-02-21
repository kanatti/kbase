mod commands;
mod config;
mod domains;
mod output;
mod tags;
mod vault;

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum IndexType {
    Tags,
    Links,
    Search,
}

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum SortBy {
    Name,
    Count,
}

#[derive(Parser)]
#[command(
    name = "kbase",
    about = "Knowledge Base CLI for markdown vaults",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Show config
    Config,

    /// Add a new vault
    Add { name: String, path: String },

    /// Set the active vault
    Use { name: String },

    /// List all configured vaults
    Vaults,

    /// List all domains with note counts
    Domains {
        /// Field to sort by
        #[arg(long, default_value_t = SortBy::Name, value_enum)]
        sort: SortBy,
    },

    /// List notes (all, or filtered by domain or search term)
    Notes {
        /// Show only notes in this domain
        #[arg(long)]
        domain: Option<String>,

        /// Filter by name/title match
        #[arg(long)]
        term: Option<String>,

        /// Show only notes with this tag
        #[arg(long)]
        tag: Option<String>,

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

    /// List all tags
    Tags {
        /// Field to sort by
        #[arg(long, default_value_t = SortBy::Name, value_enum)]
        sort: SortBy,
    },

    /// Build search and tag indexes
    Index {
        /// Build only specific indexes (tags, links, search). Default: build all
        #[arg(long, value_enum)]
        only: Vec<IndexType>,
    },
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    commands::handle_command(cli.command)
}
