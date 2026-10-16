pub fn is_empty(s: &str) -> bool {
    s.len() == 0
}

pub fn is_ascii(s: &str) -> bool {
    for c in s.chars() {
        if c < 0 as char || c > 127 as char {
            return false
        } 
    }
    return true
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    v.split_at(index)
    
}

pub fn find(v: &str, pat: char) -> usize {
    v.find(pat).unwrap()
}