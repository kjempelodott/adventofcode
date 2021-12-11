#![feature(int_abs_diff)]

extern crate aoc2021;
use aoc2021::read_from_stdin;

fn main() {
    let mut error_score = 0;
    let mut auto_completes: Vec<usize> = vec![];
    'line: for line in read_from_stdin().lines() {
        let mut scan = vec![];
        for c in line.chars() {
            match c {
                '('|'['|'{'|'<' => scan.push(c),
                _ => {
                    if (scan.pop().unwrap_or('\0') as u8).abs_diff(c as u8) > 3 {
                        error_score += match c {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => unreachable!()
                        };
                        continue 'line
                    }
                }
            };
        }
        auto_completes.push(scan.iter()
                            .rev()
                            .map(|c| match c {
                                '(' => 1,
                                '[' => 2,
                                '{' => 3,
                                '<' => 4,
                                _ => unreachable!()
                            })
                            .fold(0, |s,v| 5*s + v));
    }
    auto_completes.sort();
    println!("Part 1: {}", error_score);
    println!("Part 2: {}", auto_completes[auto_completes.len()/2]);
}
