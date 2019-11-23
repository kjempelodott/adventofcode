#![feature(try_from)]

use std::io::{self, Read};
pub fn read_from_stdin() -> String {
    let mut buffer = String::new();
    #[allow(unused_must_use)] {
        io::stdin().read_to_string(&mut buffer);
    }
    buffer
}

pub fn into_lines(string: String) -> Vec<String> {
    string.trim()
        .lines()
        .filter_map(|s| if s.is_empty() { None } else { Some(s.to_owned()) })
        .collect::<Vec<String>>()
}

#[macro_export]
macro_rules! numbers {
    ($string:expr => $type:ty) => {
        $string
            .split(|c: char| !(c.is_digit(10) || c == '-'))
            .filter_map(|l| l.parse::<$type>().ok())
            .collect::<Vec<$type>>()
    };
}
