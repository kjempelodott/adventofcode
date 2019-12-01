#[macro_use]
extern crate adventofcode2019;
use adventofcode2019::read_from_stdin;

use std::iter::successors;

fn main() {
    let m: Vec<u64> = numbers!(read_from_stdin() => u64);
    println!("Part 1: {}", m.iter()
             .map(|n| n/3 - 2)
             .sum::<u64>());

    println!("Part 2: {}", m.iter()
             .map(|n| successors(Some(*n), |p| (p/3).checked_sub(2))
                  .skip(1)
                  .sum::<u64>())
             .sum::<u64>());
}
