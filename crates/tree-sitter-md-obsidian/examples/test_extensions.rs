use tree_sitter::Parser;

fn main() {
    println!("=== Testing Obsidian Extensions for tree-sitter-markdown ===\n");

    test_inline_extensions();
    println!("\n{}\n", "=".repeat(60));
    test_block_extensions();
}

fn test_inline_extensions() {
    println!("--- INLINE PARSING (wikilinks + tags) ---\n");

    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_md_obsidian::INLINE_LANGUAGE.into())
        .unwrap();

    // Test various tag and wikilink formats
    let test_cases = vec![
        ("Simple tag", "#simple"),
        ("Nested tag", "#project/backend/api"),
        ("Multi-word tag", "#multi-word-tag"),
        ("Multiple tags", "#tag1 #tag2 #nested/tag3"),
        ("Simple wikilink", "[[note]]"),
        ("Wikilink with alias", "[[note|My Note]]"),
        ("Wikilink with path", "[[folder/subfolder/note]]"),
        (
            "Mixed content",
            "Text with [[link]] and #tag and more [[another|text]].",
        ),
        (
            "Tags in sentence",
            "This is #important and #urgent for #project/alpha release.",
        ),
        (
            "Code should NOT parse tags",
            "`#not-a-tag` and `[[not-a-link]]`",
        ),
    ];

    for (description, content) in test_cases {
        println!("Test: {}", description);
        println!("Input: {}", content);

        let tree = parser.parse(content, None).unwrap();
        let root = tree.root_node();

        // Extract tags and wikilinks
        let mut tags = Vec::new();
        let mut wikilinks = Vec::new();

        extract_nodes(&root, content, &mut tags, &mut wikilinks);

        if !tags.is_empty() {
            println!("  Tags found: {:?}", tags);
        }
        if !wikilinks.is_empty() {
            println!("  Wikilinks found: {:?}", wikilinks);
        }
        if tags.is_empty() && wikilinks.is_empty() {
            println!("  (no tags or wikilinks found)");
        }

        println!();
    }
}

fn test_block_extensions() {
    println!("--- BLOCK PARSING (full document) ---\n");

    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_md_obsidian::LANGUAGE.into())
        .unwrap();

    let content = r#"# Project Notes

This document has [[wikilink]] and #tag in regular text.

## Code Blocks

Tags in code should be ignored:

```python
# This is a comment with #tag
print("Not a real #tag")
```

## Lists with Tags

- Item with #todo tag
- Another with #done and [[reference]]
- Third item

## More Content

Regular paragraph with #project/alpha and [[docs/api|API Documentation]].
"#;

    println!("Parsing full document...\n");
    let tree = parser.parse(content, None).unwrap();
    let root = tree.root_node();

    println!("Document structure (S-expression):");
    println!("{}\n", root.to_sexp());

    println!("Note: Block parser handles document structure.");
    println!("Inline elements (tags, wikilinks) need inline parser on text regions.");
}

fn extract_nodes(
    node: &tree_sitter::Node,
    content: &str,
    tags: &mut Vec<String>,
    wikilinks: &mut Vec<String>,
) {
    if node.kind() == "tag" {
        let text = node.utf8_text(content.as_bytes()).unwrap_or("");
        tags.push(text.to_string());
    } else if node.kind() == "wiki_link" {
        let text = node.utf8_text(content.as_bytes()).unwrap_or("");
        wikilinks.push(text.to_string());
    }

    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            extract_nodes(&cursor.node(), content, tags, wikilinks);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}
