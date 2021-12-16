extern crate aoc2021;
use aoc2021::read_from_stdin;

extern crate itertools;
use itertools::Itertools;

use std::collections::HashMap;

type P = (char, char);

fn run(poly: &mut HashMap<P,usize>, map: &HashMap<P,(P,P)>, l: char, n: usize) -> usize {
    for _ in 0..n {
        *poly = poly.iter().fold(HashMap::new(), |mut x, (p, n)| {
            let (ac, cb) = map.get(&p).unwrap();
            *x.entry(*ac).or_insert(0) += n;
            *x.entry(*cb).or_insert(0) += n;
            x
        });
    }

    let mut counts = HashMap::from([(l, 1)]);
    for (p, n) in poly.iter() {
        *counts.entry(p.0).or_insert(0) += n;
    }

    let max = counts.values().max().unwrap();
    let min = counts.values().min().unwrap();
    max - min
}

fn main() {
    let input = read_from_stdin();
    let mut lines = input.lines();
    let template = lines.next().unwrap();
    let mut poly = template.chars()
        .tuple_windows::<(char,char)>()
        .map(|p| (p, 1))
        .collect::<HashMap<P,usize>>();

    let map = lines.skip(1)
        .map(|l| {
            let c: Vec<char> = l.chars().collect();
            let ab = (c[0], c[1]);
            let ac = (c[0], c[6]);
            let cb = (c[6], c[1]);
            (ab, (ac, cb))
        })
        .collect::<HashMap<P,(P,P)>>();

    let last = template.chars().last().unwrap();
    println!("Part 1: {}", run(&mut poly, &map, last, 10));
    println!("Part 2: {}", run(&mut poly, &map, last, 30));    
}
