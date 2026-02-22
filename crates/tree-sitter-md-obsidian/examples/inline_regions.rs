use tree_sitter::{Node, Parser, Tree};

fn main() {
    println!("=== Understanding Inline Regions in Block Tree ===\n");

    let content = r##"# Project Notes

This paragraph has [[wikilink]] and #tag in it.

## Code Section

```python
# This is NOT an inline region (it's code_fence_content)
print("#not-a-tag")
```

## List Section

- List item with #todo
- Another item with [[reference]]

Regular paragraph after list with #important tag.
"##;

    println!("Content to parse:\n{}\n", content);
    println!("{}\n", "=".repeat(60));

    // Step 1: Parse with BLOCK parser
    let mut block_parser = Parser::new();
    block_parser
        .set_language(&tree_sitter_md_obsidian::LANGUAGE.into())
        .unwrap();

    let block_tree = block_parser.parse(content, None).unwrap();

    println!("Step 1: Block tree structure\n");
    print_tree_structure(&block_tree.root_node(), content, 0);
    println!("\n{}\n", "=".repeat(60));

    // Step 2: Find all (inline) nodes
    println!("Step 2: Finding all (inline) nodes in block tree\n");
    let inline_nodes = find_inline_nodes(&block_tree);

    println!("Found {} inline regions:\n", inline_nodes.len());
    for (i, node) in inline_nodes.iter().enumerate() {
        let start = node.start_byte();
        let end = node.end_byte();
        let text = &content[start..end];
        let line = node.start_position().row + 1;

        println!(
            "  Region {}: (line {}, bytes {}..{})",
            i + 1,
            line,
            start,
            end
        );
        println!("    Text: {:?}", text);
    }

    println!("\n{}\n", "=".repeat(60));

    // Step 3: Parse each inline region with INLINE parser
    println!("Step 3: Parse each inline region for tags/wikilinks\n");

    let mut inline_parser = Parser::new();
    inline_parser
        .set_language(&tree_sitter_md_obsidian::INLINE_LANGUAGE.into())
        .unwrap();

    for (i, node) in inline_nodes.iter().enumerate() {
        let start = node.start_byte();
        let end = node.end_byte();
        let inline_text = &content[start..end];

        println!("Region {}: {:?}", i + 1, inline_text);

        let inline_tree = inline_parser.parse(inline_text, None).unwrap();
        let mut tags = Vec::new();
        let mut wikilinks = Vec::new();

        extract_features(
            &inline_tree.root_node(),
            inline_text,
            &mut tags,
            &mut wikilinks,
        );

        if !tags.is_empty() {
            println!("  → Tags: {:?}", tags);
        }
        if !wikilinks.is_empty() {
            println!("  → Wikilinks: {:?}", wikilinks);
        }
        if tags.is_empty() && wikilinks.is_empty() {
            println!("  → (no tags or wikilinks)");
        }
        println!();
    }

    println!("{}\n", "=".repeat(60));
    println!("Conclusion:");
    println!("- Block tree has {} inline nodes", inline_nodes.len());
    println!("- Each inline node is a region that can contain tags/wikilinks");
    println!("- Code blocks do NOT create inline nodes (correctly ignored)");
    println!("- This approach is architecturally correct: parse only inline regions");
}

/// Recursively find all nodes with kind="inline" in the tree
fn find_inline_nodes(tree: &Tree) -> Vec<Node> {
    let mut nodes = Vec::new();
    walk_for_inline(&tree.root_node(), &mut nodes);
    nodes
}

fn walk_for_inline<'a>(node: &Node<'a>, collector: &mut Vec<Node<'a>>) {
    if node.kind() == "inline" {
        collector.push(*node);
    }

    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            walk_for_inline(&cursor.node(), collector);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

/// Print tree structure with indentation
fn print_tree_structure(node: &Node, content: &str, depth: usize) {
    let indent = "  ".repeat(depth);
    let kind = node.kind();

    // Show inline nodes specially
    let marker = if kind == "inline" {
        " ← INLINE REGION"
    } else {
        ""
    };

    // Show text for leaf nodes or inline nodes
    if node.child_count() == 0 || kind == "inline" {
        let text = node.utf8_text(content.as_bytes()).unwrap_or("");
        let preview = if text.len() > 40 {
            format!("{}...", &text[..40])
        } else {
            text.to_string()
        };
        println!("{}{}{} {:?}", indent, kind, marker, preview);
    } else {
        println!("{}{}{}", indent, kind, marker);
    }

    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            print_tree_structure(&cursor.node(), content, depth + 1);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}

/// Extract tags and wikilinks from an inline tree
fn extract_features(
    node: &Node,
    content: &str,
    tags: &mut Vec<String>,
    wikilinks: &mut Vec<String>,
) {
    let kind = node.kind();

    if kind == "tag" {
        let text = node.utf8_text(content.as_bytes()).unwrap_or("");
        tags.push(text.to_string());
    } else if kind == "wiki_link" {
        let text = node.utf8_text(content.as_bytes()).unwrap_or("");
        wikilinks.push(text.to_string());
    }

    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            extract_features(&cursor.node(), content, tags, wikilinks);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}
