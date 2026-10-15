pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32 as f64) / 1.8 as f64
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    (9_f64 / 5_f64 ) * c + 32_f64
}