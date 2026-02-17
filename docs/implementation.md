# Implementation Plan

Incremental steps — each leaves the tool in a working, useful state.

## Step 1: Config
`kb config` and `kb config set vault <path>`.  
Foundation for everything. Nothing else works without it.

## Step 2: Topics + Ls
`kb topics` and `kb ls <topic>`.  
Pure filesystem, no parsing. First real navigation.

## Step 3: Note Resolution + Read
`kb read <note>`.  
Needs the note resolution logic (short name → full path). This resolver is used by almost every command after this.

## Step 4: Outline
`kb outline <note>`.  
Needs basic heading parser on a single file. First use of parsing.

## Step 5: Search
`kb search`.  
Ripgrep wrapper with topic scoping. Most useful command day-to-day.

## Step 6: Tasks
`kb tasks` and `kb task`.  
Ripgrep for finding tasks, file mutation for marking done.

## Step 7: Parser + Index
Parse every note into a `Note` struct, build the in-memory index.  
No new commands — this is the engine for everything below.

## Step 8: Link Graph
`kb links`, `kb backlinks`, `kb orphans`, `kb deadends`, `kb unresolved`.  
All powered by the index.

## Step 9: Tags
`kb tags`, `kb tag`.  
Also powered by the index.

## Step 10: Write Ops
`kb new`, `kb append`, `kb daily`.

## Step 11: Stats + Report
`kb stats`, `kb report`.  
Compose everything above.

## Step 12: Research
`kb research`.  
Capstone — composes navigation + search + tasks + links.
