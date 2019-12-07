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

pub mod metric {
    use std::convert::TryFrom;
    #[derive(Copy,Clone,Debug,Default,PartialEq,Eq,PartialOrd,Ord,Hash)]
    pub struct Point {
        pub y: isize,
        pub x: isize,
    }
    #[derive(Copy,Clone,Debug,PartialEq,Eq,PartialOrd,Ord)]
    pub enum Direction {
        Up,
        Left,
        Right,
        Down
    }
    use self::Direction::*;

    impl Point {
        pub fn new<T>(y: T, x: T) -> Self where isize: TryFrom<T> {
            Point { y: TryFrom::try_from(y).unwrap_or(0),
                    x: TryFrom::try_from(x).unwrap_or(0) }
        }
        pub fn mov(&mut self, direction: Direction) {
            match direction {
                Up    => self.y -= 1,
                Left  => self.x -= 1,
                Right => self.x += 1,
                Down  => self.y += 1
            }
        }
        pub fn next_to(&self, direction: Direction) -> Point {
            match direction {
                Up    => Point::new(self.y - 1, self.x),
                Left  => Point::new(self.y, self.x - 1),
                Right => Point::new(self.y, self.x + 1),
                Down  => Point::new(self.y + 1, self.x)
            }
        }
    }
}
