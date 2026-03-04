# Repository Documentation Feature

**Purpose**: Document codebases in the vault (shareable) with local path mappings (machine-specific).

**Status**: Planning  
**Date**: 2026-02-27

---

## Problem

Agents need to:
1. Discover what codebases are relevant
2. Understand codebase structure
3. Navigate and read code files
4. Find related research notes

No structured way to document codebases currently.

---

## Solution

**Repository descriptions** → stored in vault (version-controlled)  
**Local paths** → stored in config (machine-specific)  
**Related notes** → use tags + explicit wikilinks

### Vault Structure

```
vault/
├── _repos/
│   ├── datafusion.md      ← Codebase description
│   ├── tantivy.md
│   └── kbase.md
├── rust/
│   ├── datafusion-query-planning.md    #datafusion
│   └── datafusion-memory.md            #datafusion
└── databases/
    └── datafusion-architecture.md      #datafusion
```

### Local Config

`~/.kbase/config.toml`:
```toml
[vaults.personal.repos]
datafusion = "/Users/balu/Code/datafusion"
tantivy = "/Users/balu/Code/tantivy"
```

### Connection

Filename matches config key: `_repos/datafusion.md` ↔ `repos.datafusion`

---

## Repository Description Format

`_repos/datafusion.md`:

```markdown
---
name: datafusion
language: rust
repository: https://github.com/apache/datafusion
---

# Apache DataFusion

Query engine in Rust with SQL and DataFrame APIs.

## Key Directories

- `datafusion/core/src/` - Core execution engine
- `datafusion/optimizer/src/` - Query optimization
- `datafusion/physical-plan/src/` - Physical operators

## Entry Points

- Main: `datafusion/core/src/lib.rs`
- Examples: `datafusion/examples/`

## Related Notes

Start here:
- [[databases/datafusion-architecture]] - High-level overview
- [[rust/datafusion-query-planning]] - Optimizer deep-dive

See also: All notes tagged #datafusion
```

**Frontmatter:**
- `name` - Unique identifier (required)
- `language` - Primary language (optional)
- `repository` - Git URL (optional)

**Related Notes:**
- **Explicit links** - Curate important notes in description
- **Tag convention** - Tag all related notes with `#<repo-name>`
- Use `kbase notes --tag datafusion` to find all tagged notes

---

## Commands

### `kbase repo list`

List all repositories in current vault.

```bash
kbase repo list
```

**Output:**
```
datafusion   ✓ Apache DataFusion query engine
tantivy      ✓ Full-text search library
backend-api  ⚠ path not set
```

- `✓` = local path configured
- `⚠` = description exists, path not set

**JSON:**
```bash
kbase repo list --json
```

### `kbase repo describe --name <name>`

Show repository description.

```bash
kbase repo describe --name datafusion
```

**Output:**
```
Repository: datafusion
Path: /Users/balu/Code/datafusion ✓
Language: rust
Repository: https://github.com/apache/datafusion

----------------------------------------

[... renders markdown content ...]
```

### `kbase repo configure --name <name> --path <path>`

Set local path for a repository.

```bash
kbase repo configure --name datafusion --path ~/Code/datafusion
```

**Output:**
```
✓ Configured datafusion → /Users/balu/Code/datafusion
```

### `kbase repo path --name <name>`

Get resolved local path.

```bash
kbase repo path --name datafusion
# /Users/balu/Code/datafusion
```

Use for scripts/agents to resolve paths.

---

## Workflows

### Creating a Repo Description

```bash
# Create description
cat > vault/_repos/datafusion.md << 'EOF'
---
name: datafusion
language: rust
---

# Apache DataFusion

Query engine...

## Related Notes
- [[rust/datafusion-query-planning]]
EOF

# Configure local path
kbase repo configure --name datafusion --path ~/Code/datafusion

# Commit
git add vault/_repos/datafusion.md
git commit -m "Document DataFusion"
```

### New Engineer Onboarding

```bash
# Clone vault
git clone team-vault ~/team-vault
kbase add team ~/team-vault
kbase use team

# See repos
kbase repo list

# Read descriptions
kbase repo describe --name datafusion
kbase repo describe --name backend-api

# Clone and configure
git clone https://github.com/apache/datafusion ~/Code/datafusion
kbase repo configure --name datafusion --path ~/Code/datafusion

git clone git@company:backend-api ~/Code/backend-api
kbase repo configure --name backend-api --path ~/Code/backend-api
```

### Agent Usage

User: "How does DataFusion optimize queries?"

```bash
# Discover repos
kbase repo list

# Read description (sees structure + curated links)
kbase repo describe --name datafusion
# Shows: [[rust/datafusion-query-planning]] ← curated note

# Find all related notes
kbase notes --tag datafusion
# Shows all notes tagged #datafusion

# Read curated notes first
kbase read rust/datafusion-query-planning.md

# Get path and read code
kbase repo path --name datafusion
read /Users/balu/Code/datafusion/optimizer/src/lib.rs
```

Agent combines:
- Repo description (structure)
- Curated notes (your insights)
- All tagged notes (comprehensive)
- Source code (implementation)

---

## Implementation

### Phase 1: Basic Commands

**1. Add command structure** (`src/main.rs`):
```rust
#[derive(Subcommand)]
pub enum Command {
    Repo {
        #[command(subcommand)]
        command: RepoCommand,
    },
}

#[derive(Subcommand)]
pub enum RepoCommand {
    List {
        #[arg(long)]
        json: bool,
    },
    Describe {
        #[arg(long)]
        name: String,
        #[arg(long)]
        json: bool,
    },
    Configure {
        #[arg(long)]
        name: String,
        #[arg(long)]
        path: String,
    },
    Path {
        #[arg(long)]
        name: String,
    },
}
```

**2. Create `src/repos/` module:**
- `mod.rs` - Main logic
- `types.rs` - Data structures

**3. Update `src/config.rs`:**
- Add `repos: HashMap<String, String>` to `VaultConfig`
- Methods to read/write repo paths

**4. Create `src/commands/repos.rs`:**
- Command handlers

**5. Key functions:**
- `list_repos(vault) -> Vec<RepoInfo>`
- `describe_repo(vault, config, name) -> Result<RepoDescription>`
- `configure_repo_path(config, vault_name, repo_name, path) -> Result<()>`
- `get_repo_path(config, vault_name, repo_name) -> Result<PathBuf>`

### Future Enhancements

- Auto-detect language (Cargo.toml, package.json)
- Validate paths on configure
- Clone helper using repository URL
- Wikilink integration: `[[repo:datafusion/src/lib.rs]]`

---

## Success Criteria

✅ Version-control codebase docs in vault  
✅ Each user has their own local paths  
✅ Agents discover repos + related notes naturally  
✅ Zero config conflicts between users  
✅ Works with multi-vault setup  

---

## Related

- `plan/agentic.md` - Agent-focused features
- `docs/vault.md` - Vault conventions
- `docs/tags.md` - Tag system
