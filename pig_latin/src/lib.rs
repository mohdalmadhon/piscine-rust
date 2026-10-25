pub fn pig_latin(text: &str) -> String {
    let vec: Vec<char> = text.chars().collect();
    let c: char = vec[0].to_ascii_lowercase();

    match c {
        'a' | 'e' | 'i' | 'o' | 'u' => format!("{}hay", text),
        _ => {
            if vec.len() >= 3
                && vec[1].to_ascii_lowercase() == 'q'
                && vec[2].to_ascii_lowercase() == 'a'
            {
                let res: String = text.chars().skip(3).collect();
                format!("{}{}quay", res, &text[..1])
            } else {
                let res: String = text.chars().skip(1).collect();
                format!("{}{}ay", res, c)
            }
        }
    }
}
