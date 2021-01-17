#[macro_use]
extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;

use std::collections::HashMap;

fn main() {
    let input: Vec<usize> = numbers!(read_from_stdin() => usize);
    let mut m: HashMap<usize,usize> = input.into_iter()
        .enumerate()
        .map(|(i,x)| (x,i))
        .collect();
    let mut next = 0;
    for i in m.len()..2019 {
        next = if let Some(j) = m.insert(next, i) { i-j } else { 0 }
    }
    println!("Part 1: {}", next);
    for i in 2019..30000000-1 {
        next = if let Some(j) = m.insert(next, i) { i-j } else { 0 }
    }
    println!("Part 2: {}", next);
}
