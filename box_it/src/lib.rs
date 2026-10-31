pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    let mut out: Vec<Box<u32>> = Vec::new();

    for part in s.split_whitespace() {
        let n: u32 = if let Some(num) = part.strip_suffix('k') {
            let f: f64 = num.parse().expect("Not a valid number");
            (f * 1000.0).round() as u32
        } else {
            part.parse().expect("Not a valid number")
        };
        out.push(Box::new(n));
    }
    out
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    a.into_iter().map(|b| *b).collect()
}