pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut biggest = 0;
    for (_, value) in h {
        if value > biggest {
            biggest = value;
        }
    }
    biggest
}