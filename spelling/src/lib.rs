const ONES: [&str; 20] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen",
    "seventeen", "eighteen", "nineteen",
];

const TENS: [&str; 10] = [
    "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

fn below_thousand(n: u64) -> String {
    let mut parts = Vec::new();

    let hundreds = n / 100;
    let rest = n % 100;

    if hundreds > 0 {
        parts.push(format!("{} hundred", ONES[hundreds as usize]));
    }

    if rest > 0 {
        if rest < 20 {
            parts.push(ONES[rest as usize].to_string());
        } else {
            let t = TENS[(rest / 10) as usize];
            let o = rest % 10;
            if o == 0 {
                parts.push(t.to_string());
            } else {
                parts.push(format!("{}-{}", t, ONES[o as usize]));
            }
        }
    }

    parts.join(" ")
}

pub fn spell(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let mut parts = Vec::new();

    let millions = n / 1_000_000;
    let thousands = (n / 1_000) % 1_000;
    let rest = n % 1_000;

    if millions > 0 {
        parts.push(format!("{} million", below_thousand(millions)));
    }
    if thousands > 0 {
        parts.push(format!("{} thousand", below_thousand(thousands)));
    }
    if rest > 0 {
        parts.push(below_thousand(rest));
    }

    parts.join(" ")
}