use std::io::ErrorKind;

pub fn open_file(s: &str) -> File {
    match File::open(s) {
        Ok(file) => file,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            panic!("File not found: {}", s)
        }
        Err(e) => panic!("Problem opening the file: {:?}", e),
    }
}