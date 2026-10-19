pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let mut start_of_word = true;

    for c in input.chars() {
        if c.is_whitespace() {
            result.push(c);
            start_of_word = true;
        } else if start_of_word {
            result.extend(c.to_uppercase());
            start_of_word = false;
        } else {
            result.push(c);
        }
    }

    result
}

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().collect::<String>()
            } else {
                c.to_uppercase().collect::<String>()
            }
        })
        .collect()
}