# Vault

A **vault** is a directory containing markdown notes organized into top-level domain folders. Each vault is configured in `~/.kbase/config.toml` and can be accessed via `kbase` commands.

## Structure

```
kanatti-notes/              ← vault root
├── elasticsearch/          ← domain
│   ├── 01-home.md
│   └── esql-analysis.md
├── lucene/                 ← domain
│   ├── 01-home.md
│   ├── search-flow.md
│   └── codecs.md
├── __templates/            ← excluded (starts with __)
├── _logs/                  ← excluded (starts with _)
└── 01-home.md              ← root-level note (no domain)
```

**What gets included:**
- All `.md` files in domain folders
- Domain folders (top-level dirs not starting with `_` or `.`)
- Root-level `.md` files (exist but have no domain)

**What gets excluded:**
- Directories starting with `_` (e.g., `_logs/`, `_planning/`)
- Directories starting with `__` (e.g., `__templates/`, `__attachments/`)
- Hidden files/directories (starting with `.`)
- Non-markdown files

## Domains

A **domain** is a top-level directory in the vault. See [domains.md](domains.md) for details.

```bash
kbase domains                  # list all domains
kbase domains --sort count     # sort by note count
```

## Notes

Individual markdown files are called **notes**. See [notes.md](notes.md) for details.

```bash
kbase notes                    # list all notes
kbase notes --domain lucene    # notes in a domain
kbase notes --tag deep-dive    # notes with a tag (requires index)
```

**Title extraction:**
- Notes display their first `# Heading` as the title
- Scans first 20 lines of each file
- Falls back to filename stem if no heading found

## Index Storage

Vault indexes are stored separately from the vault content:

```
~/.kbase/
├── config.toml
└── <vault-name>/
    ├── tags.json           # tag → note paths mapping
    └── search.tantivy/     # future: full-text index
```

Each vault has its own index directory. Indexes are built by `kbase index` and used by tag-related commands.

**Building the index:**

```bash
kbase index                    # scan vault and build indexes
```

This creates/updates `tags.json` with mappings from tags to note paths. Required for `kbase notes --tag` and `kbase tags` commands.
