pub fn delete_and_backspace(s: &mut String) {
    let mut str = String::new();
    let mut chars = s.chars().peekable();
    let mut counter: i32 = 0;
    for c in chars {
        if counter != 0 && c != '+' {
            counter -= 1;
            continue;
        }
        if c == '-' && str.len() != 0 {
            str.pop();
        } else if c == '+' {
            counter += 1;
        } else {
            str.push(c);
        }
    }
    
    s.clear();
    s.push_str(&str);
}


pub fn do_operations(v: &mut [String]) {
    for operation in v.iter_mut() {
        if let Some((left, right)) = operation.split_once('+') {
            let result = left.parse::<i32>().unwrap()
                + right.parse::<i32>().unwrap();
            *operation = result.to_string();
        } else if let Some((left, right)) = operation.split_once('-') {
            let result = left.parse::<i32>().unwrap()
                - right.parse::<i32>().unwrap();
            *operation = result.to_string();
        }
    }
}