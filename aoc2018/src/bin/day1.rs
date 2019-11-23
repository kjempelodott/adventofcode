#[macro_use]
extern crate adventofcode2018;
use adventofcode2018::read_from_stdin;

use std::collections::BTreeSet;

fn main() {
    let df: Vec<isize> = numbers!(read_from_stdin() => isize);
    println!("Part 1: {}", df.iter().sum::<isize>());

    let mut f = 0;
    let mut seen = BTreeSet::new();
    let mut it = df.iter().cycle();
    while seen.insert(f) {
        f += it.next().unwrap();
    }
    println!("Part 2 {}", f);
}
