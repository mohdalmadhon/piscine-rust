pub fn arrange_phrase(phrase: &str) -> String {
    let parts: Vec<&str> = phrase.split(' ').collect();
    let n = parts.len();
    let mut v: Vec<String> = vec![String::new(); n];

    for w in parts {
        for c in w.chars() {
            if c >= '0' && c <= '9' {
                v[c.to_digit(10).unwrap() as usize - 1] =
                    w.replace(c, "");
            }
        }
    }

    v.join(" ")
}