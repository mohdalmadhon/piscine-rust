fn all_upper(s: &str) -> bool {
    let mut has_letter = false;

    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            has_letter = true;
            if !c.is_ascii_uppercase() {
                return false;
            }
        }
    }

    has_letter
}

pub fn talking(text: &str) -> &str {
    let s = text.trim();

    if s.is_empty() {
        return "Just say something!";
    }

    let yelling = all_upper(s);
    let question = s.ends_with('?');

    if yelling && question {
        "Quiet, I am thinking!"
    } else if yelling {
        "There is no need to yell, calm down!"
    } else if question {
        "Sure."
    } else {
        "Interesting"
    }
}