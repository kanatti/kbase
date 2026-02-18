# kb - Knowledge Base CLI

Fast, tag-aware knowledge base for markdown vaults.

## What is kb?

kb is a knowledge base that organizes markdown notes using domains and hashtags. Filter notes by domain (`rust/`, `docker/`) or tags (`#wip`, `#bug-fix`) to quickly find what you need.

## Installation

```bash
cargo install --path .
```

## Setup

```bash
kb add my-notes ~/notes     # Add vault
kb use my-notes             # Set active vault  
kb index                    # Build tag index
kb notes --tag rust         # Start filtering
```

## Commands

```bash
# Vault management
kb config                   # Show configuration
kb add name /path           # Add vault  
kb use name                 # Switch vault
kb vaults                   # List vaults

# Note operations
kb domains                  # List domains
kb notes                    # List all notes
kb notes --domain rust      # Filter by domain
kb notes --tag wip          # Filter by tag
kb notes --tag rust --files # Filenames only
kb tags                     # List all tags
kb read rust/basics.md      # View note
kb index                    # Rebuild index
```

## Temporary Vault Switching

Use `KB_VAULT` to temporarily override the active vault:

```bash
# Normal usage (uses active vault)
kb notes --tag bug

# Temporary switch by vault name  
KB_VAULT=work kb notes --tag bug
KB_VAULT=personal kb index

# Still permanent switching
kb use work                 # Change active vault
```

## Structure

kb works with existing folder structures:

```
vault/
├── rust/
│   └── basics.md           # Contains #rust #wip tags
├── lucene/
│   └── search.md           # Contains #deep-dive tag
└── notes.md
```

Top-level folders become domains. Hashtags in markdown files are automatically indexed.

## Tagging

Use hashtags naturally in markdown:

```markdown
# Rust Ownership
This covers #rust #memory-management concepts.
Status: #wip
```

Filter by tags:

```bash
kb notes --tag rust         # All rust notes
kb notes --tag wip          # Work-in-progress
kb notes --domain rust --tag advanced  # Combine filters
```

## License

MIT