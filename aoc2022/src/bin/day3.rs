extern crate aoc2022;
use aoc2022::{into_lines,read_from_stdin};

fn main() {
    let mut sum = 0usize;
    let mut sum2 = 0usize;
    let rucksacks = into_lines(read_from_stdin());
    for rs in rucksacks.iter() {
        let (c1, c2) = rs.as_bytes().split_at(rs.len() / 2);
        let same = *c1.iter().find(|i| c2.contains(i)).unwrap() as usize;
        sum += same - if same > 90 { 96 } else { 38 };
    }
    for g in rucksacks.chunks(3) {
        let (rs1, rs2, rs3) = (g[0].as_bytes(), g[1].as_bytes(), g[2].as_bytes());
        let same = *rs1.iter().find(|i| rs2.contains(i) && rs3.contains(i)).unwrap() as usize;
        sum2 += same - if same > 90 { 96 } else { 38 };
    }
    println!("Part 1: {}", sum);
    println!("Part 2: {}", sum2);
}
