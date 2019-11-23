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

pub mod metric {
    use std::convert::TryFrom;
    #[derive(Copy,Clone,Debug,Default,PartialEq,Eq,PartialOrd,Ord,Hash)]
    pub struct Point {
        pub y: isize,
        pub x: isize,
    }

    #[derive(Copy,Clone,Debug,Default,PartialEq,Eq,PartialOrd,Ord,Hash)]
    pub struct Point3D {
        pub z: isize,
        pub y: isize,
        pub x: isize,
    }

    #[derive(Copy,Clone,Debug,Default,PartialEq,Eq,PartialOrd,Ord,Hash)]
    pub struct Point4D {
        pub w: isize,
        pub z: isize,
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

    impl Point4D {
        pub fn new<T>(w: T, z: T, y: T, x: T) -> Self where isize: TryFrom<T> {
            Point4D { w: TryFrom::try_from(w).unwrap_or(0),
                      z: TryFrom::try_from(z).unwrap_or(0),
                      y: TryFrom::try_from(y).unwrap_or(0),
                      x: TryFrom::try_from(x).unwrap_or(0) }
        }
    }

    impl Point3D {
        pub fn new<T>(z: T, y: T, x: T) -> Self where isize: TryFrom<T> {
            Point3D { z: TryFrom::try_from(z).unwrap_or(0),
                      y: TryFrom::try_from(y).unwrap_or(0),
                      x: TryFrom::try_from(x).unwrap_or(0) }
        }
    }

    pub trait Manhattan where Self: Sized + Clone + Copy {
        fn adjacent(&self) -> [Self; 4];
        fn distance(&self, other: &Self) -> usize;
        fn nearest(&self, other: &mut Vec<Self>) -> Option<Self> {
            other.sort_by_key(|&p| self.distance(&p));
            match other.len() {
                0 => None,
                1 => Some(other[0]),
                _ => {
                    if self.distance(&other[0]) == self.distance(&other[1]) {
                        None
                    } else {
                        Some(other[0])
                    }
                }
            }
        }
    }

    use std::unimplemented;
    impl Manhattan for Point3D {
        fn adjacent(&self) -> [Self; 4] {
            unimplemented!();
        }
        fn distance(&self, other: &Point3D) -> usize {
            (self.x - other.x).abs() as usize + (self.y - other.y).abs() as usize +
            (self.z - other.z).abs() as usize
        }
    }

    impl Manhattan for Point4D {
        fn adjacent(&self) -> [Self; 4] {
            unimplemented!();
        }
        fn distance(&self, other: &Point4D) -> usize {
            (self.x - other.x).abs() as usize + (self.y - other.y).abs() as usize +
            (self.z - other.z).abs() as usize + (self.w - other.w).abs() as usize
        }
    }

    pub trait Euclidean where Self: Sized {
        fn adjacent(&self) -> [Self; 8];
    }

    impl Euclidean for Point {
        fn adjacent(&self) -> [Self; 8] {
            [Point::new(self.y - 1, self.x - 1),
             Point::new(self.y - 1, self.x    ),
             Point::new(self.y - 1, self.x + 1),
             Point::new(self.y,     self.x - 1),
             Point::new(self.y,     self.x + 1),
             Point::new(self.y + 1, self.x - 1),
             Point::new(self.y + 1, self.x    ),
             Point::new(self.y + 1, self.x + 1)]
        }
    }
}
