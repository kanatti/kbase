use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Resolve a wikilink target to an actual note path.
///
/// # Resolution Strategy
///
/// **Path-style (contains '/'):**
/// 1. Try absolute from vault root: `target.md`
/// 2. Try relative to source domain: `source_domain/target.md`
/// 3. Fail if neither exists
///
/// **Bare name (no '/'):**
/// 1. Try same domain as source: `source_domain/target.md`
/// 2. Try root level: `target.md`
/// 3. Fail if neither exists
///
/// # Examples
///
/// Source: `lucene/search-flow.md`
///
/// - `[[codecs]]` → `lucene/codecs.md` (same domain)
/// - `[[glossary]]` → `glossary.md` (root level, if not in lucene/)
/// - `[[internals/codec]]` → `lucene/internals/codec.md` (relative path)
/// - `[[lucene/codecs]]` → `lucene/codecs.md` (absolute path)
/// - `[[datafusion/query]]` → `datafusion/query.md` (absolute path)
pub fn resolve_target(
    target: &str,
    source_path: &Path,
    all_notes: &HashSet<PathBuf>,
) -> Option<PathBuf> {
    // Case 1: Path-style (contains '/')
    if target.contains('/') {
        // Try absolute from vault root
        let abs_candidate = PathBuf::from(format!("{}.md", target));
        if all_notes.contains(&abs_candidate) {
            return Some(abs_candidate);
        }

        // Try relative to source domain
        if let Some(domain) = source_path.parent() {
            let rel_candidate = domain.join(format!("{}.md", target));
            if all_notes.contains(&rel_candidate) {
                return Some(rel_candidate);
            }
        }

        // Path-style must match exactly (absolute or relative)
        return None;
    }

    // Case 2: Bare name (no '/')
    
    // 2a. Try same domain as source
    if let Some(domain) = source_path.parent() {
        let candidate = domain.join(format!("{}.md", target));
        if all_notes.contains(&candidate) {
            return Some(candidate);
        }
    }

    // 2b. Try root level
    let root_candidate = PathBuf::from(format!("{}.md", target));
    if all_notes.contains(&root_candidate) {
        return Some(root_candidate);
    }

    // Not found
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_vault() -> HashSet<PathBuf> {
        // Simulate vault structure
        let notes = vec![
            "lucene/search-flow.md",
            "lucene/codecs.md",
            "lucene/internals/codec-details.md",
            "datafusion/query-execution.md",
            "datafusion/01-home.md",
            "lucene/01-home.md",
            "glossary.md", // Root-level note
        ];

        notes.iter().map(PathBuf::from).collect()
    }

    #[test]
    fn test_bare_name_same_domain() {
        let all_notes = setup_vault();
        let source = PathBuf::from("lucene/search-flow.md");

        // [[codecs]] from lucene/search-flow.md → lucene/codecs.md
        let result = resolve_target("codecs", &source, &all_notes);
        assert_eq!(result, Some(PathBuf::from("lucene/codecs.md")));
    }

    #[test]
    fn test_bare_name_cross_domain_fails() {
        let all_notes = setup_vault();
        let source = PathBuf::from("lucene/search-flow.md");

        // [[query-execution]] from lucene/ → NOT in lucene/, NOT at root → fails
        let result = resolve_target("query-execution", &source, &all_notes);
        assert_eq!(result, None);
    }

    #[test]
    fn test_bare_name_same_domain_priority() {
        let all_notes = setup_vault();
        let source = PathBuf::from("lucene/search-flow.md");

        // [[01-home]] from lucene/ → lucene/01-home.md (same domain wins)
        let result = resolve_target("01-home", &source, &all_notes);
        assert_eq!(result, Some(PathBuf::from("lucene/01-home.md")));
    }

    #[test]
    fn test_bare_name_from_root_no_cross_domain() {
        let all_notes = setup_vault();
        let source = PathBuf::from("glossary.md"); // Root-level source

        // [[01-home]] from root → exists in lucene/ and datafusion/ but NOT at root → fails
        let result = resolve_target("01-home", &source, &all_notes);
        assert_eq!(result, None);
    }

    #[test]
    fn test_bare_name_not_found() {
        let all_notes = setup_vault();
        let source = PathBuf::from("lucene/search-flow.md");

        // [[nonexistent]] → not found
        let result = resolve_target("nonexistent", &source, &all_notes);
        assert_eq!(result, None);
    }

    #[test]
    fn test_absolute_path() {
        let all_notes = setup_vault();
        let source = PathBuf::from("lucene/search-flow.md");

        // [[lucene/codecs]] → lucene/codecs.md (absolute)
        let result = resolve_target("lucene/codecs", &source, &all_notes);
        assert_eq!(result, Some(PathBuf::from("lucene/codecs.md")));

        // [[datafusion/query-execution]] → datafusion/query-execution.md (absolute)
        let result = resolve_target("datafusion/query-execution", &source, &all_notes);
        assert_eq!(result, Some(PathBuf::from("datafusion/query-execution.md")));
    }

    #[test]
    fn test_relative_path() {
        let all_notes = setup_vault();
        let source = PathBuf::from("lucene/search-flow.md");

        // [[internals/codec-details]] from lucene/ → lucene/internals/codec-details.md (relative)
        let result = resolve_target("internals/codec-details", &source, &all_notes);
        assert_eq!(result, Some(PathBuf::from("lucene/internals/codec-details.md")));
    }

    #[test]
    fn test_path_style_not_found() {
        let all_notes = setup_vault();
        let source = PathBuf::from("lucene/search-flow.md");

        // [[nonexistent/note]] → not found (path-style has no fallback)
        let result = resolve_target("nonexistent/note", &source, &all_notes);
        assert_eq!(result, None);
    }

    #[test]
    fn test_root_level_note() {
        let all_notes = setup_vault();
        let source = PathBuf::from("lucene/search-flow.md");

        // [[glossary]] → glossary.md (root level fallback)
        let result = resolve_target("glossary", &source, &all_notes);
        assert_eq!(result, Some(PathBuf::from("glossary.md")));
    }

    #[test]
    fn test_root_level_source() {
        let all_notes = setup_vault();
        let source = PathBuf::from("glossary.md");

        // [[codecs]] from root-level note → NOT at root, NOT in same domain → fails
        let result = resolve_target("codecs", &source, &all_notes);
        assert_eq!(result, None);
    }
}
