# kb Configuration

## Location

```
~/.kb/config.toml
```

## Format

```toml
default = "personal"

[vaults]
personal = "~/Documents/kanatti-notes"
work     = "~/Documents/work-notes"
```

`default` names which vault to use when no `--vault` flag is given.
Each key under `[vaults]` is a short name you choose; the value is the path (tilde-expanded).

## Vault Resolution Order

For every command, the vault is resolved in this order — first match wins:

| Priority | Source | Example |
|---|---|---|
| 1 | `KB_VAULT` env var | `KB_VAULT=/tmp/testvault kb notes` |
| 2 | `--vault <name>` flag | `kb --vault work notes` |
| 3 | `default` in config | `default = "personal"` |

`KB_VAULT` takes a **raw path** (not a vault name). Used for tests and one-off overrides.
`--vault` takes a **vault name** from the config's `[vaults]` table.

## Vault Management

```bash
kb vault list                             # show all vaults, mark default with *
kb vault add <name> <path>               # add or update a vault
kb vault default <name>                  # change the default
kb vault remove <name>                   # remove a vault (does not delete files)
```

Examples:

```bash
kb vault add personal ~/Documents/kanatti-notes
kb vault add work ~/Documents/work-notes
kb vault default personal
kb vault list
  * personal  ~/Documents/kanatti-notes
    work      ~/Documents/work-notes
```

## Other Config

```bash
kb config          # show full config and resolved vault path
```

No other config keys for now. Future options (editor, pager, etc.) will go here.

## First-Time Setup

```bash
$ kb domains
Error: no vaults configured. Run `kb vault add <name> <path>` to get started.

$ kb vault add personal ~/Documents/kanatti-notes
Added vault "personal" → ~/Documents/kanatti-notes
Set as default.

$ kb domains
...
```

The first vault added is automatically set as the default.
