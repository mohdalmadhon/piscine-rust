pub fn number_logic(num: u32) -> bool {
    let length = num.to_string().len();
    let nums: Vec<char> = num.to_string().chars().collect();
    let mut sum: u32 = 0;

    for c in nums {
        let n = c.to_digit(10).unwrap().pow(length as u32);
        sum += n;
    }

    sum == num
}
