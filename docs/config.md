# kb Configuration

## Location

```
~/.kb/config.toml
```

## Format

```toml
active_vault = "personal"

[vaults.personal]
path = "~/Documents/kanatti-notes"

[vaults.work]
path = "~/Documents/work-notes"
```

`active_vault` names which vault to use by default.
Each key under `[vaults]` is a vault name; the value contains the path.

## Vault Management

```bash
kb config                        # show current config
kb add <name> <path>            # add a vault
kb use <name>                   # set active vault
kb vaults                       # list all vaults
```

Examples:

```bash
kb add personal ~/Documents/kanatti-notes
kb add work ~/Documents/work-notes  
kb use personal
kb vaults
  personal (active) → ~/Documents/kanatti-notes
  work             → ~/Documents/work-notes
```

## First-Time Setup

```bash
$ kb domains
Error: No config found. Run `kb add <name> <path>` to add a vault.

$ kb add personal ~/Documents/kanatti-notes
Added vault 'personal' to config
Set as active vault

$ kb domains
...
```

The first vault added is automatically set as active.
