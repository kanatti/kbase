use tabled::{builder::Builder, settings::Style};

/// Print a table with any number of columns.
pub fn print_table(headers: &[&str], rows: &[Vec<String>]) {
    if rows.is_empty() {
        return;
    }

    let mut builder = Builder::default();
    builder.push_record(headers.iter().copied());
    for row in rows {
        builder.push_record(row);
    }
    
    let mut table = builder.build();
    table.with(Style::psql());
    println!("{}", table);
}
