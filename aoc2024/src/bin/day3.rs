use regex::Regex;
use aoc2024::read_from_stdin;

fn calc(input: &str) -> isize {
    let mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    mul.captures_iter(&input)
        .map(|c| c.extract())
        .map(|(_, [n,m])| n.parse::<isize>().unwrap() * m.parse::<isize>().unwrap())
        .sum::<isize>()
}

fn main() {
    let input = read_from_stdin();
    println!("Part 1: {}", calc(&input));

    let spl: Vec<&str> = input.split("don't()").collect();
    println!("Part 2: {}", calc(spl[0]) + spl[1..].iter()
             .filter_map(|s| s.split_once("do()"))
             .map(|(_,dos)| calc(dos))
             .sum::<isize>());
}
