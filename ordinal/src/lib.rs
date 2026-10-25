pub fn num_to_ordinal(x: u32) -> String {
    let vec: Vec<char> = x.to_string().chars().collect();
    let c: &char = vec.get(vec.len() - 1).unwrap();
    
    if x == 11 {
        return String::from("11th");
    } else if x == 12 {
        return String::from("12th");
    } else if x == 13 {
        return String::from("13th");
    }
    
    let str: String = match *c {
        '1' => String::from("st"),
        '2' => String::from("nd"),
        '3' => String::from("rd"),
        _ => String::from("th")
    };
    
    let mut result = x.to_string();
    result.push_str(&str);
    result
}