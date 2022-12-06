extern crate aoc2022;
use aoc2022::read_from_stdin;

use std::collections::HashSet;

fn main() {
    let signals: Vec<_> = read_from_stdin().chars().collect();
    let (n, _) = signals.windows(4)
        .enumerate()
        .find(|(_, w)| w.into_iter().collect::<HashSet<&char>>().len() == 4)
        .unwrap();
    println!("Part 1: {}", n + 4);
    let (n, _) = signals.windows(14)
        .enumerate()
        .find(|(_, w)| w.into_iter().collect::<HashSet<&char>>().len() == 14)
        .unwrap();
    println!("Part 2: {}", n + 14);
}
