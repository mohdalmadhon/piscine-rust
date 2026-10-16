pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    (c, 2_i32.pow(c as u32) as f64, (c as f64).abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let parts: Vec<&str> = a.split_whitespace().collect();
    let mut vec: Vec<String> = Vec::new();

    for c in &parts {
        let n: i32 = c.parse().unwrap();
        vec.push((n as f64).exp().to_string());
    }

    (a, vec.join(" "))
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut vec: Vec<f64> = Vec::new();

    for n in &b {
        vec.push((n.abs() as f64).ln());
    }

    (b, vec)
}