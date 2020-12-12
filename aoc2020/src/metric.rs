use std::convert::TryFrom;
use std::ops::{BitAnd,BitOr};

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
    Down,
    Diagonal(u8)
}

impl From<u8> for Direction {
    fn from(n: u8) -> Self {
        match n {
            1 => Up,
            2 => Left,
            4 => Right,
            8 => Down,
            _ => Diagonal(n)
        }
    }
}


impl From<Direction> for u8 {
    fn from(d: Direction) -> Self {
        match d {
            Up    => 1,
            Left  => 2,
            Right => 4,
            Down  => 8,
            Diagonal(n) => n
        }
    }
}

impl BitOr for Direction {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Direction::from(u8::from(self) | u8::from(rhs))
    }
}

impl BitAnd for Direction {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Direction::from(u8::from(self) & u8::from(rhs))
    }
}

use self::Direction::*;

impl Point {
    pub fn new<T>(y: T, x: T) -> Self where isize: TryFrom<T> {
        Point { y: TryFrom::try_from(y).unwrap_or(0),
                x: TryFrom::try_from(x).unwrap_or(0) }
    }
}

pub trait Euclidean<const D: usize> where Self: Sized {
    fn adjacent(&self) -> [Self; D];
    fn mov(&mut self, direction: Direction);
}

impl Euclidean<8> for Point {
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
    fn mov(&mut self, dir: Direction) {
        if dir & Up == Up {
            self.y -= 1;
        }
        else if dir & Down == Down {
            self.y += 1;
        }
        if dir & Left  == Left {
            self.x -= 1;
        }
        else if dir & Right  == Right {
            self.x += 1;
        }
    }
}
