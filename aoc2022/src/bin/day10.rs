extern crate aoc2022;
use aoc2022::{into_lines,read_from_stdin};

fn main() {
    let input = into_lines(read_from_stdin());
    let mut lines  = input.iter();
    let mut op = None;
    let mut x: isize = 1;
    let mut sum = 0;
    let mut row = String::new();
    println!("Part 2:");
    for i in 1..=240 {
        row.push( if (x - ((i-1)  % 40)).abs() <= 1 { '#' } else { '.' } );
        match i % 40 {
            0 => {
                println!("{}", row);
                row.clear();
            },
            20 => { sum += i * x; },
            _ => {}
        }
        if let Some(v) = op {
            x += v;
            op = None;
        }
        else if let Some(line) = lines.next() {
            op = line.split_once(' ').and_then(|(_, n)| n.parse::<isize>().ok());
        }
    }
    println!("Part 1: {}", sum);
}
