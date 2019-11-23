extern crate adventofcode2018;
extern crate itertools;
use adventofcode2018::read_from_stdin;
use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    let mut visited: HashSet<(u16, u16)> = HashSet::new();
    let mut visited_twice: HashSet<(u16, u16)> = HashSet::new();

    let claims = read_from_stdin()
        .lines()
        .map(|line| {
            let sq: Vec<u16> = line.split(|c: char| !c.is_numeric())
                .filter_map(|e| e.parse::<u16>().ok())
                .skip(1)
                .collect();
            (sq[0], sq[0] + sq[2], sq[1], sq[1] + sq[3])
        })
        .inspect(|&(x0, x1, y0, y1)| {
            for (x,y) in (x0..x1).cartesian_product(y0..y1) {
                if !visited.insert((x,y)) {
                    visited_twice.insert((x,y));
                }
            }
        }).collect::<Vec<(u16,u16,u16,u16)>>();

    let id = claims
        .iter()
        .take_while(|&&(x0, x1, y0, y1)| {
            (x0..x1).cartesian_product(y0..y1)
                .any(|(x,y)| visited_twice.contains(&(x,y)))
        })
        .count();

    println!("Part 1: {}", visited_twice.len());
    println!("Part 2: {}", id + 1);
}
