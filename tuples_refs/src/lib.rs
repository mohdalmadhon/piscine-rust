pub struct Student<'a>(i32, &'a str, &'a str);

pub fn id(student: &Student) -> u32 {
    student.0 as u32
}

pub fn first_name(student: &Student) -> &str {
    student.1
}

pub fn last_name(student: &Student) -> &str {
    student.2
}