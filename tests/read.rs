mod common;
use common::{kbase, setup_vault};

// ---------------------------------------------------------------------------
// Expected outputs (constants for reuse)
// ---------------------------------------------------------------------------

const EXPECTED_READ_WITH_LINE_NUMBERS: &str = r#"     1	# Search Flow Deep Dive
     2	
     3	How TermQuery flows through IndexSearcher, Weight, Scorer and into TopDocs.
     4	
     5	This is a #deep-dive into #lucene #indexing and #performance optimization.
     6	
     7	## Phase 1: IndexSearcher.search()
     8	
     9	Entry point for all searches in Lucene. This covers #search-internals.
    10	
    11	### Step 1: createWeight()
    12	
    13	Weight wraps the query for reuse across segments.
    14	
    15	### Step 2: BulkScorer
    16	
    17	Scores documents in bulk for a segment.
    18	
    19	## Phase 2: Scoring
    20	
    21	Final BM25 scoring and TopDocs collection. The #scoring algorithm is #wip.
"#;

const EXPECTED_OUTLINE_WITH_LINE_NUMBERS: &str = r#"     1	# Search Flow Deep Dive
     7	  ## Phase 1: IndexSearcher.search()
    11	    ### Step 1: createWeight()
    15	    ### Step 2: BulkScorer
    19	  ## Phase 2: Scoring
"#;

// ---------------------------------------------------------------------------
// kb read — raw content
// ---------------------------------------------------------------------------

#[test]
fn read_note_outputs_raw_content() {
    let tmp = setup_vault();
    let output = kbase(&tmp)
        .args(["read", "lucene/search-flow.md"])
        .output()
        .unwrap();

    assert!(output.status.success(), "expected exit 0");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("# Search Flow Deep Dive"));
    assert!(stdout.contains("## Phase 1: IndexSearcher.search()"));
    assert!(stdout.contains("### Step 1: createWeight()"));
}

#[test]
fn read_root_level_note() {
    let tmp = setup_vault();
    let output = kbase(&tmp).args(["read", "01-home.md"]).output().unwrap();

    assert!(output.status.success(), "expected exit 0");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("# Home"));
}

// ---------------------------------------------------------------------------
// kb read --outline
// ---------------------------------------------------------------------------

#[test]
fn read_outline_indents_by_level() {
    let tmp = setup_vault();
    let output = kbase(&tmp)
        .args(["read", "lucene/search-flow.md", "--outline"])
        .output()
        .unwrap();

    assert!(output.status.success(), "expected exit 0");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    // H1 — no indent
    assert!(lines.contains(&"# Search Flow Deep Dive"));
    // H2 — 2 spaces
    assert!(lines.contains(&"  ## Phase 1: IndexSearcher.search()"));
    assert!(lines.contains(&"  ## Phase 2: Scoring"));
    // H3 — 4 spaces
    assert!(lines.contains(&"    ### Step 1: createWeight()"));
    assert!(lines.contains(&"    ### Step 2: BulkScorer"));
}

#[test]
fn read_outline_excludes_body_text() {
    let tmp = setup_vault();
    let output = kbase(&tmp)
        .args(["read", "lucene/search-flow.md", "--outline"])
        .output()
        .unwrap();

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Body prose should not appear
    assert!(!stdout.contains("How TermQuery flows"));
    assert!(!stdout.contains("Entry point for all searches"));
}

// ---------------------------------------------------------------------------
// kb read -n / --line-numbers
// ---------------------------------------------------------------------------

#[test]
fn read_with_line_numbers_short_form() {
    let tmp = setup_vault();
    let output = kbase(&tmp)
        .args(["read", "lucene/search-flow.md", "-n"])
        .output()
        .unwrap();

    assert!(output.status.success(), "expected exit 0");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout, EXPECTED_READ_WITH_LINE_NUMBERS);
}

#[test]
fn read_with_line_numbers_long_form() {
    let tmp = setup_vault();
    let output = kbase(&tmp)
        .args(["read", "lucene/search-flow.md", "--line-numbers"])
        .output()
        .unwrap();

    assert!(output.status.success(), "expected exit 0");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout, EXPECTED_READ_WITH_LINE_NUMBERS);
}

#[test]
fn read_outline_with_line_numbers_short_form() {
    let tmp = setup_vault();
    let output = kbase(&tmp)
        .args(["read", "lucene/search-flow.md", "--outline", "-n"])
        .output()
        .unwrap();

    assert!(output.status.success(), "expected exit 0");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout, EXPECTED_OUTLINE_WITH_LINE_NUMBERS);
}

#[test]
fn read_outline_with_line_numbers_long_form() {
    let tmp = setup_vault();
    let output = kbase(&tmp)
        .args([
            "read",
            "lucene/search-flow.md",
            "--outline",
            "--line-numbers",
        ])
        .output()
        .unwrap();

    assert!(output.status.success(), "expected exit 0");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!(stdout, EXPECTED_OUTLINE_WITH_LINE_NUMBERS);
}

// ---------------------------------------------------------------------------
// kb read — error handling
// ---------------------------------------------------------------------------

#[test]
fn read_missing_note_errors() {
    let tmp = setup_vault();
    let output = kbase(&tmp)
        .args(["read", "lucene/nonexistent.md"])
        .output()
        .unwrap();

    assert!(!output.status.success(), "expected non-zero exit");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("note not found: lucene/nonexistent.md"),
        "stderr was: {stderr}"
    );
}

#[test]
fn read_wrong_domain_errors() {
    let tmp = setup_vault();
    let output = kbase(&tmp)
        .args(["read", "no-such-domain/note.md"])
        .output()
        .unwrap();

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("note not found: no-such-domain/note.md"));
}
