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
