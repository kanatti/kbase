# Configuration

kbase stores configuration in a TOML file that tracks all your vaults and which one is currently active.

## Location

`~/.kbase/config.toml` (or `$KBASE_HOME/config.toml`)

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
kbase config              # Show config and all vaults
kbase add <name> <path>   # Add a vault (path must exist, supports ~)
kbase use <name>          # Set active vault
kbase vaults              # List all vaults
```

The first vault added is automatically set as active.

## Environment Variables

- `KBASE_HOME` - Override config directory (default: `~/.kbase`)
- `KBASE_VAULT` - Override active vault for a single command

```bash
KBASE_VAULT=work kbase domains    # Use 'work' vault temporarily
```

## Index Storage

Indexes are stored per-vault in `~/.kbase/<vault-name>/`:
- `tags.json` - Tag index
- `search.tantivy/` - Search index
