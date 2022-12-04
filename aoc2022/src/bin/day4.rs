extern crate aoc2022;
use aoc2022::read_from_stdin;

fn main() {
    let (s1, s2) = read_from_stdin().lines().fold((0,0), |(s1, s2), line| {
        let s = line.split(|c: char| !c.is_digit(10))
            .filter_map(|l| l.parse::<usize>().ok())
            .collect::<Vec<_>>();
        let (l1, r1, l2, r2) = (s[0], s[1], s[2], s[3]);
        if (l1 <= l2 && r1 >= r2) || (l2 <= l1 && r2 >= r1) {
            return (s1 + 1, s2 + 1)
        }
        if (r2 >= l1 && l2 <= r1) || (l2 >= r1 && r2 <= l1) || (r1 >= l2 && l1 <= r2) || (l1 >= r2 && r1 <= l2) {
            return (s1, s2 + 1)
        }
        (s1, s2)
    });
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
