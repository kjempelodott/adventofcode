#![feature(int_abs_diff)]

#[macro_use]
extern crate aoc2021;
use aoc2021::read_from_stdin;

fn main() {
    let n: Vec<u64> = numbers!(read_from_stdin() => u64);
    let min = *n.iter().min().unwrap();
    let max = *n.iter().max().unwrap();
    println!("Part 1: {}", (min..max)
             .map(|i| n.iter()
                  .map(|&x| x.abs_diff(i))
                  .sum::<u64>())
             .min().unwrap());
    println!("Part 2: {}", (min..max)
             .map(|i| n.iter()
                  .map(|&x| (1..x.abs_diff(i)+1).sum::<u64>())
                  .sum::<u64>())
             .min().unwrap());
}
