/// Print a two-column table with headers and aligned columns.
///
/// The left column width is calculated to fit both the header and all data rows.
/// Columns are separated by two spaces.
pub fn print_table(headers: (&str, &str), rows: &[(String, String)]) {
    let (left_header, right_header) = headers;

    // Calculate max width needed for left column (header or data)
    let data_width = rows.iter().map(|(left, _)| left.len()).max().unwrap_or(0);
    let width = data_width.max(left_header.len());

    // Print header
    println!("{:<width$}  {}", left_header, right_header, width = width);

    // Print rows
    for (left, right) in rows {
        println!("{:<width$}  {}", left, right, width = width);
    }
}

/// Print a three-column table with headers and aligned columns.
///
/// Column widths are calculated to fit headers and all data rows.
/// Columns are separated by two spaces.
pub fn print_table3(headers: (&str, &str, &str), rows: &[(String, String, String)]) {
    let (h1, h2, h3) = headers;

    // Calculate max width needed for each column
    let w1 = rows
        .iter()
        .map(|(c1, _, _)| c1.len())
        .max()
        .unwrap_or(0)
        .max(h1.len());
    let w2 = rows
        .iter()
        .map(|(_, c2, _)| c2.len())
        .max()
        .unwrap_or(0)
        .max(h2.len());

    // Print header
    println!("{:<w1$}  {:<w2$}  {}", h1, h2, h3, w1 = w1, w2 = w2);

    // Print rows
    for (c1, c2, c3) in rows {
        println!("{:<w1$}  {:<w2$}  {}", c1, c2, c3, w1 = w1, w2 = w2);
    }
}
