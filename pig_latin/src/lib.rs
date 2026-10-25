pub fn pig_latin(text: &str) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    if chars.is_empty() {
        return String::new();
    }

    if "aeiou".contains(chars[0].to_ascii_lowercase()) {
        return format!("{}ay", text);
    }

    while i < chars.len() && !"aeiou".contains(chars[i].to_ascii_lowercase()) {
        if chars[i].to_ascii_lowercase() == 'q'
            && i + 1 < chars.len()
            && chars[i + 1].to_ascii_lowercase() == 'u'
        {
            i += 2;
            break;
        }
        i += 1;
    }

    let (first, rest): (String, String) = (
        chars[..i].iter().collect(),
        chars[i..].iter().collect(),
    );

    format!("{}{}ay", rest, first)
}