use anyhow::{bail, Result};
use std::path::PathBuf;

use crate::config::Config;
use crate::output;
use crate::repos::{list_repos, read_repo_description};
use crate::vault::Vault;
use crate::RepoCommand;

pub fn handle_repo(vault: &Vault, command: RepoCommand) -> Result<()> {
    match command {
        RepoCommand::List { json } => handle_list(vault, json),
        RepoCommand::Describe { name, json } => handle_describe(vault, &name, json),
        RepoCommand::Configure { name, path } => handle_configure(vault, &name, &path),
    }
}

fn handle_list(vault: &Vault, json: bool) -> Result<()> {
    let config = Config::load()?;
    let repos = list_repos(vault, &config)?;

    if json {
        let json_repos: Vec<_> = repos
            .iter()
            .map(|r| {
                serde_json::json!({
                    "name": r.name,
                    "brief": r.brief,
                    "local_path": r.local_path,
                    "configured": r.local_path.is_some(),
                })
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&json_repos)?);
        return Ok(());
    }

    if repos.is_empty() {
        println!("No repositories found.");
        println!("Create repository descriptions in _repos/ directory.");
        return Ok(());
    }

    let rows: Vec<_> = repos
        .iter()
        .map(|r| {
            let path_available = if r.local_path.is_some() {
                "✓ yes"
            } else {
                "⚠ no"
            };
            (r.name.clone(), path_available.to_string(), r.brief.clone())
        })
        .collect();

    output::print_table3(("Repository", "Path Available", "Description"), &rows);
    Ok(())
}

fn handle_describe(vault: &Vault, name: &str, json: bool) -> Result<()> {
    let config = Config::load()?;
    let content = read_repo_description(vault, name)?;
    let local_path = config.get_repo_path(name);

    if json {
        let data = serde_json::json!({
            "name": name,
            "content": content,
            "local_path": local_path,
            "configured": local_path.is_some(),
        });
        println!("{}", serde_json::to_string_pretty(&data)?);
        return Ok(());
    }

    // Print header
    println!("Repository: {}", name);
    
    if let Some(path) = local_path {
        println!("Path: {}", path.display());
    } else {
        println!("Path: ⚠ not set");
    }
    
    println!();
    println!("{}", "─".repeat(40));
    println!();
    
    // Print content
    print!("{}", content);
    
    Ok(())
}

fn handle_configure(vault: &Vault, name: &str, path: &str) -> Result<()> {
    // Verify repo exists in vault
    read_repo_description(vault, name)?;

    // Resolve and validate path
    let expanded = PathBuf::from(shellexpand::tilde(path).as_ref());
    if !expanded.exists() {
        bail!("Path does not exist: {}", expanded.display());
    }

    // Update config
    let mut config = Config::load()?;
    config.set_repo_path(name.to_string(), expanded.clone())?;
    config.save()?;

    println!("✓ Configured {} → {}", name, expanded.display());
    Ok(())
}


