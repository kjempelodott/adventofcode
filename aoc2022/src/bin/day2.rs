extern crate aoc2022;
use aoc2022::read_from_stdin;

fn main() {
    let mut part1 = 0usize;
    let mut part2 = 0usize;
    for line in read_from_stdin().lines() {
        let bytes = line.as_bytes();
        // Normalize to 0,1,2 => R,P,S / Win,Draw,Lose
        let (abc, xyz) = (bytes[0] as i8 - 65, bytes[2] as i8 - 88);

        let score1 = xyz + 1;
        let result1 = 3 * ((4 + xyz - abc) % 3);

        let score2 = 1 + (3 + (xyz - 1) + abc) % 3;
        let result2 = 3 * (xyz % 3);

        part1 += result1 as usize + score1 as usize;
        part2 += result2 as usize + score2 as usize;
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
