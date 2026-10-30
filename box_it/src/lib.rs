use std::mem;

pub fn parse_into_boxed(s: String) -> Vec<Box<u32>>{
    let parts: Vec<&str> = s.split(' ').collect();
    let mut out: Vec<Box<u32>> = Vec::new();
    
    for str in parts {
        if str.ends_with('k') {
            let mut s1: String = String::from(str);
            s1.pop();
            let num: f64 = s1.parse().expect("Not a valid number");
            let n: u32 = (num * 1000 as f64) as u32;
            out.push(Box::new(n));
        }
    }
    out
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    let mut vec: Vec<u32> = Vec::new();
    for r in a {
        vec.push(*r);
    }
    vec
}
