pub fn is_pangram(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    for i in 97..=122 {
        let c: char = (i as u8) as char;
        let ch: &char = &c;
        if !chars.contains(&ch.to_ascii_lowercase()) {
            return false;
        }
    }
    return true;
}