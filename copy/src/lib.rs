pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (
        c,
        (c as f64).exp(),
        (c as f64).abs().ln(),
    )
}

pub fn str_function(a: String) -> (String, String) {
    let result = a
        .split_whitespace()
        .map(|s| s.parse::<f64>().unwrap().exp().to_string())
        .collect::<Vec<String>>()
        .join(" ");

    (a, result)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let result = b
        .iter()
        .map(|n| (*n as f64).abs().ln())
        .collect::<Vec<f64>>();

    (b, result)
}