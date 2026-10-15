pub fn sum(x: u8, y: u8) -> u16 {
    (x + y) as u16
}

pub fn diff(x: i16, y:i16) -> i32 {
    (x - y) as i32
}

pub fn pro(x: i8, y:i8) -> i32 {
    (x * y) as i32
}

pub fn quo(x:i32, y:i32) -> i32 {
    x / y 
}

pub fn rem(x:i32, y:i32) -> i32 {
    x % y
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_sum() {
//         println!("sum: {}", sum(234, 2)); // 'sum: 236'
//         println!("sum: {}", sum(1, 255));
    
//     }
// }