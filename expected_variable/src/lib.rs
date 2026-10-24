use case::CaseExt;

pub fn edit_distance(source: &str, target: &str) -> usize {
    let s: Vec<char> = source.chars().collect();
    let t: Vec<char> = target.chars().collect();
    let mut prev: Vec<usize> = (0..=t.len()).collect();

    for i in 1..=s.len() {
        let mut curr = vec![i; t.len() + 1];
        for j in 1..=t.len() {
            let cost = if s[i - 1] == t[j - 1] { 0 } else { 1 };
            curr[j] = (prev[j] + 1).min(curr[j - 1] + 1).min(prev[j - 1] + cost);
        }
        prev = curr;
    }
    prev[t.len()]
}

pub fn expected_variable(compared: &str, expected: &str) -> Option<String> {
    let only_word_chars = !compared.is_empty()
        && compared.chars().all(|c| c.is_alphanumeric() || c == '_');

    let is_camel = !compared.contains('_')
        && (compared.is_camel_lowercase() || compared == compared.to_camel());
    let lower = compared.to_lowercase();
    let is_snake = lower.contains('_') && lower == lower.to_snake();

    if !only_word_chars || !(is_camel || is_snake) {
        return None;
    }

    let expected = expected.to_lowercase();
    let distance = edit_distance(&lower, &expected) as f64;
    let len = expected.chars().count() as f64;
    let alikeness = (len - distance) / len * 100.0;

    if alikeness > 50.0 {
        Some(format!("{}%", alikeness.round() as i64))
    } else {
        None
    }
}