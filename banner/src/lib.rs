use std::{collections::HashMap, num::ParseFloatError};

pub struct Flag<'a> {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: &'a str,
}

impl<'a> Flag<'a> {
    pub fn opt_flag(name: &'a str, d: &'a str) -> Self {
        Flag {
            short_hand: format!("-{}", &name[0..1]),
            long_hand: format!("--{}", name),
            desc: d,
        }
    }
}

pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand, func);
        self.flags.insert(flag.long_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        let func = self.flags.get(input).ok_or("flag not found".to_string())?;
        func(argv[0], argv[1]).map_err(|e| e.to_string())
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a: f64 = a.parse()?;
    let b: f64 = b.parse()?;
    Ok((a / b).to_string())
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let a: f64 = a.parse()?;
    let b: f64 = b.parse()?;
    Ok((a % b).to_string())
}