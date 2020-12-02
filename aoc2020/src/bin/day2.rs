extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;

fn main() {
    let (mut policy1, mut policy2) = (0, 0);
    for line in read_from_stdin().lines() {
        let tok: Vec<&str> = line.split(|c: char| !c.is_alphanumeric()).collect();
        let (x, y) = (
            tok[0].parse::<usize>().unwrap(),
            tok[1].parse::<usize>().unwrap()
        );
        let (l, pw) = (
            tok[2].chars().nth(0).unwrap(),
            tok[4]
        );
        let count = tok[4].chars().filter(|&c| c == l).count();
        if x <= count && count <= y {
            policy1 += 1;
        }
        if (pw.chars().nth(x-1) == Some(l)) ^ (pw.chars().nth(y-1) == Some(l)) {
            policy2 += 1;
        }
    }
    println!("Part 1: {}", policy1);
    println!("Part 2: {}", policy2);
}
