extern crate aoc2022;
use aoc2022::read_from_stdin;

fn main() {
    let mut calories = vec![];
    let mut sum = 0;
    for line in read_from_stdin().lines() {
        if line.is_empty() {
            calories.push(sum);
            sum = 0;
            continue;
        }
        sum += line.parse::<usize>().unwrap();
    }
    calories.sort();
    println!("Part 1: {}", calories.last().unwrap());
    println!("Part 2: {}", calories.iter().rev().take(3).sum::<usize>());
}
