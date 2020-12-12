extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;

extern crate parse_display;
use parse_display::{Display,FromStr};

use std::convert::TryFrom;

#[derive(Default,Debug)]
struct Vector(isize, isize);

impl Vector {
    fn rotate(&mut self, deg: isize) {
        match deg {
            90 => {
                let tmp = self.0;
                self.0 = -self.1;
                self.1 = tmp;
            },
            180 => {
                self.0 = -self.0;
                self.1 = -self.1;
            },
            270 => {
                let tmp = -self.0;
                self.0 = self.1;
                self.1 = tmp;
            },
            _ => unreachable!()
        }
    }
}

#[derive(Display,FromStr)]
#[display("{}{0}")]
enum Action {
    N(isize),
    S(isize),
    E(isize),
    W(isize),
    L(isize),
    R(isize),
    F(isize)
}

use Action::*;

fn main() {
    let mut ship1 = (0, 0);
    let mut direction = Vector(0, 1);

    let mut ship2 = (0, 0);
    let mut wp = Vector(-1, 10);

    for a in read_from_stdin().lines().map(|l| l.parse::<Action>().unwrap()) {
        match a {
            N(n) => {
                ship1.0 -= n;
                wp.0 -= n;
            },
            S(n) => {
                ship1.0 += n;
                wp.0 += n;
            },
            E(n) => {
                ship1.1 += n;
                wp.1 += n;
            },
            W(n) => {
                ship1.1 -= n;
                wp.1 -= n;
            },           
            F(n) => {
                ship1.0 += n * direction.0;
                ship1.1 += n * direction.1;
                ship2.0 += n * wp.0;
                ship2.1 += n * wp.1;
            },
            R(n) => {
                direction.rotate(360-n);
                wp.rotate(360-n);
            },
            L(n) => {
                direction.rotate(n);
                wp.rotate(n);
            },
            _ => unreachable!()
        }
    }
    println!("Part 1: {}", ship1.0.abs() + ship1.1.abs());
    println!("Part 2: {}", ship2.0.abs() + ship2.1.abs());
}
