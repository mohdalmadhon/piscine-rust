pub fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }
    
    let mut a = 0;
    let mut b = 1;
    
    for _ in 2..=n {
        let next = a + b;
        a = b;
        b = next;
    }
    
    b
}