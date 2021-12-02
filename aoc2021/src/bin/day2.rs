extern crate aoc2021;
use aoc2021::read_from_stdin;

extern crate parse_display;
use parse_display::{Display,FromStr};

#[derive(Display,FromStr,PartialEq)]
#[display("{} {0}")]
#[display(style="lowercase")]
enum Instruction {
    Forward(u64),
    Up(u64),
    Down(u64)
}

use Instruction::*;

fn main() {
    let (mut x, mut z, mut w) = (0,0,0);
    for line in read_from_stdin().lines() {
        let i = line.parse::<Instruction>().unwrap();
        match i {
            Forward(v) => {
                x += v;
                w += z*v;
            },
            Up(v) => {
                z -= v;
            },
            Down(v) => {
                z += v;
            }
        };
    }
    println!("Part 1: {}", x*z);
    println!("Part 2: {}", x*w);
}
