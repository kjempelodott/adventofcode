extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;

fn policy1(x: usize, y: usize, l: char, pw: &str) -> bool {
    let c = pw.chars().filter(|&c| c == l).count();
    x <= c && c <= y
}

fn policy2(x: usize, y: usize, l: char, pw: &str) -> bool {
    (pw.chars().nth(x-1) == Some(l))^(pw.chars().nth(y-1) == Some(l))
}

fn main() {
    let (mut p1, mut p2) = (0, 0);
    for line in read_from_stdin().lines() {
        let p: Vec<&str> = line.split_whitespace().collect();
        let mm: Vec<usize> = p[0].split('-').map(|c| c.parse::<usize>().unwrap()).collect();
        let letter = p[1].chars().nth(0).unwrap();
        if policy1(mm[0], mm[1], letter, p[2]) { p1 += 1; }
        if policy2(mm[0], mm[1], letter, p[2]) { p2 += 1; }
    }
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
