pub fn first_subword(s: String) -> String {
    let mut str = String::new();

    for (i, c) in s.chars().enumerate() {
        if i != 0 && (c == '_' || c.is_ascii_uppercase()) {
            return str;
        }
        str.push(c);
    }

    str
}