# Configuration

kb stores configuration in a TOML file that tracks all your vaults and which one is currently active.

## Location

`~/.kb/config.toml` (or `$KB_HOME/config.toml`)

## Format

```toml
active_vault = "personal"

[vaults.personal]
path = "/Users/you/Documents/personal-notes"

[vaults.work]
path = "/Users/you/Documents/work-notes"
```

## Commands

```bash
kb config              # Show config and all vaults
kb add <name> <path>   # Add a vault (path must exist, supports ~)
kb use <name>          # Set active vault
kb vaults              # List all vaults
```

The first vault added is automatically set as active.

## Environment Variables

- `KB_HOME` - Override config directory (default: `~/.kb`)
- `KB_VAULT` - Override active vault for a single command

```bash
KB_VAULT=work kb domains    # Use 'work' vault temporarily
```

## Index Storage

Indexes are stored per-vault in `~/.kb/<vault-name>/`:
- `tags.json` - Tag index
- `search.tantivy/` - Search index
