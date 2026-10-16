pub fn initials(v: Vec<&str>) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();

    for word in &v {
        let mut parts = word.split_whitespace();

        let first = parts.next().unwrap();
        let last = parts.next().unwrap();

        let w = format!(
            "{}. {}.",
            first.chars().next().unwrap(),
            last.chars().next().unwrap()
        );

        vec.push(w);
    }

    vec
}