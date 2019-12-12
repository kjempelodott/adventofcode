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

#[derive(Copy,Clone,Debug,PartialEq,Eq,PartialOrd,Ord)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down
}
use self::Direction::*;

impl Direction {
    pub fn turn_left(&self) -> Self {
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up
        }
    }
    pub fn turn_right(&self) -> Self {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up
        }
    }
}

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

impl Point3D {
    pub fn new<T>(z: T, y: T, x: T) -> Self where isize: TryFrom<T> {
        Point3D { z: TryFrom::try_from(z).unwrap_or(0),
                  y: TryFrom::try_from(y).unwrap_or(0),
                  x: TryFrom::try_from(x).unwrap_or(0) }
    }
    pub fn mov(&mut self, v: &Point3D) {
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }
}
