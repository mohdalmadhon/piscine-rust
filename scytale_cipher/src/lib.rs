pub fn scytale_cipher(message: &str, i: usize) -> String {
    let chars: Vec<char> = message.chars().collect();
    if i == 0 || chars.is_empty() {
        return message.to_string();
    }

    // Number of rows (loops of the strip around the cylinder)
    let rows = (chars.len() + i - 1) / i;
    let mut result = String::with_capacity(rows * i);

    // Write the message row by row, read it column by column.
    // Empty cells are filled with spaces.
    for col in 0..i {
        for row in 0..rows {
            result.push(*chars.get(row * i + col).unwrap_or(&' '));
        }
    }

    result.trim_end().to_string()
}