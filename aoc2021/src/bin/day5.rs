extern crate aoc2021;
use aoc2021::read_from_stdin;

extern crate parse_display;
use parse_display::{Display,FromStr};

use std::collections::HashSet;

#[derive(Display,FromStr,PartialEq)]
#[display("{x0},{y0} -> {x1},{y1}")]
struct Line {
    x0: u16,
    y0: u16,
    x1: u16,
    y1: u16,
}

impl Line {
    fn plot(self) -> Box<dyn Iterator<Item=(u16,u16)>> {
        let mut rev = false;
        let y_iter = if self.y1 > self.y0 {
            self.y0..self.y1 + 1
        }
        else {
            rev ^= true;
            self.y1..self.y0 + 1
        };
        let x_iter = if self.x1 > self.x0 {
            self.x0..self.x1 + 1
        }
        else {
            rev ^= true;
            self.x1..self.x0 + 1
        };
        if self.x0 == self.x1 {
            return Box::new(y_iter.map(move |y| (y,self.x0)));
        }
        else if self.y0 == self.y1 {
            return Box::new(x_iter.map(move |x| (self.y0,x)));
        }
        if rev {
            return Box::new(y_iter.rev().zip(x_iter));
        }
        return Box::new(y_iter.zip(x_iter));
    }
}

fn main() {
    let mut vents = HashSet::new();
    let mut dvents = HashSet::new();
    let mut overlaps = HashSet::new();
    let mut doverlaps = HashSet::new();
    for line in read_from_stdin().lines() {
        let l = line.parse::<Line>().unwrap();
        let diag = l.x0 != l.x1 && l.y0 != l.y1;
        if diag {
            l.plot().for_each(|(y,x)| {
                if !dvents.insert((y,x)) {
                    doverlaps.insert((y,x));
                }
            });
        }
        else {
            l.plot().for_each(|(y,x)| {
                if !vents.insert((y,x)) {
                    overlaps.insert((y,x));
                }
            });
        }
    }
    println!("Part 1: {}", overlaps.len());
    println!("Part 2: {}", (&(&overlaps | &doverlaps) | &(&vents & &dvents)).len());
}
