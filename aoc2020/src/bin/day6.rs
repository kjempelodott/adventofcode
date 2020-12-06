#![feature(iterator_fold_self)]
extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;

use std::collections::HashSet;

fn main() {
    let input = read_from_stdin();
    println!("Part 1: {}", input.split("\n\n")
             .map(|b| b.chars()
                  .filter(|c| !c.is_whitespace())
                  .collect::<HashSet<char>>()
                  .len())
             .sum::<usize>());
    println!("Part 2: {}", input.trim().split("\n\n")
             .map(|b| b.split("\n")
                  .map(|p| p.chars().collect::<HashSet<char>>())
                  .fold_first(|is, h| is.intersection(&h).cloned().collect())
                  .unwrap()
                  .len())
             .sum::<usize>());
}
