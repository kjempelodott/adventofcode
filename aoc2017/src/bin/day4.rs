// Keywords: duplicate strings, anagrams

#[macro_use]
extern crate adventofcode2017;
extern crate itertools;

use itertools::Itertools;

fn solve_part1(words: &Vec<Vec<String>>) -> usize {
    words.iter()
        .filter(|w| w.len() == w.iter().unique().count())
        .count()
}

fn solve_part2(words: Vec<Vec<String>>) -> usize {
    words.iter()
        .map(|w| w.iter().map(|w| w.chars().sorted()).collect::<Vec<_>>())
        .filter(|w| w.len() == w.iter().unique().count())
        .count()
}

fn main() {
    let words: Vec<Vec<String>> =
        such_table!(include_str!("../../res/day4") => String);
    println!("Part 1: {}", solve_part1(&words));
    println!("Part 2: {}", solve_part2(words));
}
