extern crate aoc2022;
use aoc2022::read_from_stdin;

fn main() {
    let mut calories = vec![];
    read_from_stdin().lines().fold(0, |sum, line| {
        if line.is_empty() {
            calories.push(sum);
            return 0
        }
        sum + line.parse::<usize>().unwrap()
    });
    calories.sort();
    println!("Part 1: {}", calories.last().unwrap());
    println!("Part 2: {}", calories.iter().rev().take(3).sum::<usize>());
}
