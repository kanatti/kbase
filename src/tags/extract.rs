use std::collections::HashSet;
use regex::Regex;
use once_cell::sync::Lazy;

/// Compiled regex for extracting #tags, initialized once.
static TAG_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"#([a-zA-Z0-9][a-zA-Z0-9_-]*)").unwrap()
});

/// Extract all inline #tags from markdown content, sorted and deduplicated.
/// Ignores purely numeric tags (like #20298, #123).
pub fn extract_tags(content: &str) -> Vec<String> {
    let mut tags = HashSet::new();
    tags.extend(extract_inline_tags(content));
    
    let mut tag_vec: Vec<String> = tags.into_iter()
        .filter(|tag| !tag.chars().all(|c| c.is_numeric())) // Filter out pure numbers
        .collect();
    tag_vec.sort();
    tag_vec
}

/// Find #tags while tracking ``` code block boundaries.
fn extract_inline_tags(content: &str) -> Vec<String> {
    let mut tags = Vec::new();
    let mut in_code_block = false;
    
    for line in content.lines() {
        // Toggle code block state on ```
        if line.trim_start().starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }
        
        // Skip lines inside code blocks
        if in_code_block {
            continue;
        }
        
        // Extract #tags from this line
        for cap in TAG_REGEX.captures_iter(line) {
            tags.push(cap[1].to_string()); // Without # prefix
        }
    }
    
    tags
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_simple_tags() {
        let content = "This is a #test and #example note.";
        let tags = extract_tags(content);
        assert_eq!(tags, vec!["example", "test"]);
    }
    
    #[test]
    fn test_skip_code_blocks() {
        let content = "This has #real-tag outside code.

```rust
// This has #fake-tag inside code
let x = \"#also-fake\";
```

And #another-real tag after.";
        let tags = extract_tags(content);
        assert_eq!(tags, vec!["another-real", "real-tag"]);
    }
    
    #[test]
    fn test_various_tag_formats() {
        let content = "Tags: #year2024 #rust_lang #deep-dive #123test #mix4d2";
        let tags = extract_tags(content);
        assert_eq!(tags, vec!["123test", "deep-dive", "mix4d2", "rust_lang", "year2024"]);
    }
    
    #[test]
    fn test_no_tags() {
        let content = "This has no tags at all.";
        let tags = extract_tags(content);
        assert!(tags.is_empty());
    }
    
    #[test]
    fn test_ignores_numeric_tags() {
        let content = "PR #20298 and issue #123 but keep #bug-report and #v1_2_3";
        let tags = extract_tags(content);
        assert_eq!(tags, vec!["bug-report", "v1_2_3"]);
    }
}