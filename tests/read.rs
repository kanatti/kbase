mod common;
use common::{kb, setup_vault};

// ---------------------------------------------------------------------------
// kb read — raw content
// ---------------------------------------------------------------------------

#[test]
fn read_note_outputs_raw_content() {
    let tmp = setup_vault();
    let output = kb(&tmp)
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
    let output = kb(&tmp)
        .args(["read", "01-home.md"])
        .output()
        .unwrap();

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
    let output = kb(&tmp)
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
    let output = kb(&tmp)
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
// kb read — error handling
// ---------------------------------------------------------------------------

#[test]
fn read_missing_note_errors() {
    let tmp = setup_vault();
    let output = kb(&tmp)
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
fn read_wrong_topic_errors() {
    let tmp = setup_vault();
    let output = kb(&tmp)
        .args(["read", "no-such-topic/note.md"])
        .output()
        .unwrap();

    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("note not found: no-such-topic/note.md"));
}
