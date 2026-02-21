# kbase - Knowledge Base CLI

Fast, tag-aware knowledge base for markdown vaults.

## What is kbase?

kbase is a knowledge base that organizes markdown notes using domains and hashtags. Filter notes by domain (`rust/`, `docker/`) or tags (`#wip`, `#bug-fix`) to quickly find what you need.

## Installation

```bash
cargo install --path .
```

## Setup

```bash
kbase add my-notes ~/notes     # Add vault
kbase use my-notes             # Set active vault  
kbase index                    # Build tag index
kbase notes --tag rust         # Start filtering
```

## Commands

```bash
# Vault management
kbase config                   # Show configuration
kbase add name /path           # Add vault  
kbase use name                 # Switch vault
kbase vaults                   # List vaults

# Note operations
kbase domains                  # List domains
kbase notes                    # List all notes
kbase notes --domain rust      # Filter by domain
kbase notes --tag wip          # Filter by tag
kbase notes --tag rust --files # Filenames only
kbase tags                     # List all tags
kbase read rust/basics.md      # View note
kbase index                    # Rebuild index
```

## Temporary Vault Switching

Use `KBASE_VAULT` to temporarily override the active vault:

```bash
# Normal usage (uses active vault)
kbase notes --tag bug

# Temporary switch by vault name  
KBASE_VAULT=work kbase notes --tag bug
KBASE_VAULT=personal kbase index

# Still permanent switching
kbase use work                 # Change active vault
```

## Structure

kbase works with existing folder structures:

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
kbase notes --tag rust         # All rust notes
kbase notes --tag wip          # Work-in-progress
kbase notes --domain rust --tag advanced  # Combine filters
```

## License

MIT
