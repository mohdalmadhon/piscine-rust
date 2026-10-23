use std::fs::OpenOptions;
use std::path::Path;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let mut file = OpenOptions::new()
    .append(true)
    .create(true)
    .open(path);
    
    file.write_all(content.as_bytes()).unwrap();
}