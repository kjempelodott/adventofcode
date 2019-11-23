// Keywords: minmax, combinations

#[macro_use]
extern crate adventofcode2017;
extern crate itertools;
use itertools::Itertools;

fn solve_part1(table: &Vec<Vec<u32>>) -> u32 {
    table.iter()
        .map(|row| {
            let mut row = row.clone();
            row.sort();
            ja!(row.last()) - ja!(row.first())
        }).sum()
}

fn solve_part2(table: &Vec<Vec<u32>>) -> u32 {
    table.iter()
        .map(|row| {
            let mut row = row.clone();
            row.sort();
            ja!(row.into_iter().combinations(2).find(|xy| xy[1] % xy[0] == 0))
        })
        .map(|xy| xy[1]/xy[0])
        .sum()
}

fn main() {
    let input = such_table!(include_str!("../../res/day2") => u32);
    println!("Checksum 1: {}", solve_part1(&input));
    println!("Checksum 2: {}", solve_part2(&input));
}
