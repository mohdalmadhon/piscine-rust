pub fn is_pangram(s: &str) -> bool {
    let chars: Vec<char> = s.to_ascii_lowercase().chars().collect();

    for i in 97..=122 {
        let c = i as u8 as char;

        if !chars.contains(&c) {
            return false;
        }
    }

    true
}