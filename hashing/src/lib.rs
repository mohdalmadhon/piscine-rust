use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let sum: i32 = list.iter().sum();
    sum as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut sorted = list.to_vec();
    sorted.sort();

    let mid = sorted.len() / 2;
    if sorted.len() % 2 == 0 {
        (sorted[mid - 1] + sorted[mid]) / 2
    } else {
        sorted[mid]
    }
}

pub fn mode(list: &[i32]) -> i32 {
    let mut counts = HashMap::new();
    for &n in list {
        *counts.entry(n).or_insert(0) += 1;
    }

    let mut best = list[0];
    let mut best_count = 0;
    for (&n, &count) in &counts {
        if count > best_count || (count == best_count && n < best) {
            best = n;
            best_count = count;
        }
    }
    best
}