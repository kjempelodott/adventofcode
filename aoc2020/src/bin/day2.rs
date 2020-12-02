extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;

extern crate parse_display;
use parse_display::{Display,FromStr};

#[derive(FromStr,Display)]
#[display("{x}-{y} {c}: {password}")]
struct Input {
    x: usize,
    y: usize,
    c: char,
    password: String
}

fn main() {
    let (mut policy1, mut policy2) = (0, 0);
    for line in read_from_stdin().lines() {
        let i = line.parse::<Input>().unwrap();
        let pw = i.password.as_bytes();
        let count = pw.iter()
            .filter(|&&c| c == i.c as u8)
            .count();
        if i.x <= count && count <= i.y {
            policy1 += 1;
        }
        if (pw[i.x-1] == i.c as u8) ^ (pw[i.y-1] == i.c as u8) {
            policy2 += 1;
        }
    }
    println!("Part 1: {}", policy1);
    println!("Part 2: {}", policy2);
}
