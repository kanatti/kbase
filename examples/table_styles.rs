use tabled::{Table, Tabled, settings::Style};

#[derive(Tabled)]
struct Note {
    path: String,
    title: String,
}

fn main() {
    let notes = vec![
        Note { path: "lucene/home.md".to_string(), title: "Lucene Home".to_string() },
        Note { path: "lucene/codec.md".to_string(), title: "Codec Architecture".to_string() },
        Note { path: "rust/ownership.md".to_string(), title: "Ownership Rules".to_string() },
    ];

    println!("=== Blank (no lines) ===");
    let mut table = Table::new(&notes);
    table.with(Style::blank());
    println!("{}", table);

    println!("\n=== Empty (truly minimal) ===");
    let mut table = Table::new(&notes);
    table.with(Style::empty());
    println!("{}", table);

    println!("\n=== Psql (postgres style) ===");
    let mut table = Table::new(&notes);
    table.with(Style::psql());
    println!("{}", table);

    println!("\n=== Re-structured text ===");
    let mut table = Table::new(&notes);
    table.with(Style::re_structured_text());
    println!("{}", table);
    
    println!("\n=== Markdown ===");
    let mut table = Table::new(&notes);
    table.with(Style::markdown());
    println!("{}", table);

    println!("\n=== Rounded ===");
    let mut table = Table::new(&notes);
    table.with(Style::rounded());
    println!("{}", table);

    println!("\n=== Sharp (ASCII) ===");
    let mut table = Table::new(&notes);
    table.with(Style::sharp());
    println!("{}", table);
}
