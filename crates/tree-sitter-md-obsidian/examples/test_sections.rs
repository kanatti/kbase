use tree_sitter::Parser;

fn main() {
    println!("=== Testing Wikilinks with Sections ===\n");

    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_md_obsidian::INLINE_LANGUAGE.into())
        .unwrap();

    // Test various wikilink formats with sections
    let test_cases = vec![
        ("Simple section", "[[note#Introduction]]"),
        ("Path with section", "[[folder/note#Overview]]"),
        ("Section with alias", "[[note#Section|Display Text]]"),
        ("Path, section, alias", "[[domain/note#Details|See Details]]"),
        ("Multi-word section", "[[note#Multi Word Heading]]"),
        ("No section", "[[note]]"),
        ("Just alias", "[[note|Alias]]"),
    ];

    for (description, content) in test_cases {
        println!("Test: {}", description);
        println!("Input: {}", content);

        let tree = parser.parse(content, None).unwrap();
        let root = tree.root_node();

        // Print S-expression to see structure
        println!("  Tree: {}", root.to_sexp());
        
        // Extract wikilink components
        extract_wikilink(&root, content);
        println!();
    }
}

fn extract_wikilink(node: &tree_sitter::Node, content: &str) {
    if node.kind() == "wiki_link" {
        println!("  wiki_link node found:");
        
        let mut cursor = node.walk();
        if cursor.goto_first_child() {
            loop {
                let child = cursor.node();
                let kind = child.kind();
                let text = child.utf8_text(content.as_bytes()).unwrap_or("");
                
                println!("    {}: {:?}", kind, text);
                
                if !cursor.goto_next_sibling() {
                    break;
                }
            }
        }
    }

    let mut cursor = node.walk();
    if cursor.goto_first_child() {
        loop {
            extract_wikilink(&cursor.node(), content);
            if !cursor.goto_next_sibling() {
                break;
            }
        }
    }
}
