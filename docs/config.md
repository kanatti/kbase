# kb Configuration

## Location

Config file: `~/.kb/config.toml`

## Format

```toml
vault = "/path/to/your/vault"
```

## Commands

```bash
kb config                              # show current config and source
kb config set vault /path/to/vault    # write or update vault path
```

## Resolution Order

`kb` resolves the vault path in this order, first one wins:

1. `--vault` flag (per-call override)
2. `KB_VAULT` environment variable
3. `~/.kb/config.toml`

## First Time Setup

```bash
$ kb topics
No config found. Run `kb config set vault /path/to/vault` to get started.

$ kb config set vault ~/Documents/my-notes
Config written to ~/.kb/config.toml

$ kb topics
...
```

## Future Config Options

More options will be added here as needed.
