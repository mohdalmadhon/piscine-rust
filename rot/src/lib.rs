pub fn rotate(input: String, key: i8) -> String {
    let shift = key.rem_euclid(26) as u8;

    input
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                ((c as u8 - b'a' + shift) % 26 + b'a') as char
            } else if c.is_ascii_uppercase() {
                ((c as u8 - b'A' + shift) % 26 + b'A') as char
            } else {
                c
            }
        })
        .collect()
}