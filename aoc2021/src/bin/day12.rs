extern crate aoc2021;
use aoc2021::read_from_stdin;

use std::collections::HashMap;

type Cave<'a> = (&'a str, bool);

fn traverse<'a>(caves: &HashMap<Cave<'a>,Vec<Cave<'a>>>, m: usize) -> usize {
    let mut n = 0;
    let mut paths: Vec<(Cave,HashMap<Cave,usize>)> = vec![];
    paths.push((("start".into(), false), HashMap::new()));
    while let Some((cave, visited)) = paths.pop() {
        let connected: &Vec<Cave> = caves.get(&cave).unwrap();
        for &c in connected.iter() {
            if c.0 == "end" {
                n += 1;
            }
            else if c.1 {
                paths.push((c, visited.clone()));
            }
            else {
                let mut v = visited.clone();
                *v.entry(c).or_insert(0) += 1;
                if *v.get(&c).unwrap() > m || v.values().filter(|&&r| r > 1).count() > 1 {
                    continue;
                }
                paths.push((c, v));
            }
        }
    }
    n
}

fn main() {
    let mut caves: HashMap<Cave,Vec<Cave>> = HashMap::new();
    let input = read_from_stdin();
    for line in input.lines() {
        let (c1, c2) = line.split_once('-').unwrap();
        let (big1, big2) = (c1.to_uppercase() == c1, c2.to_uppercase() == c2);
        let (cave1, cave2) = ((c1, big1), (c2, big2));
        if c1 != "start" {
            caves.entry(cave2).or_insert(vec![]).push(cave1);
        }
        if c2 != "start" {
            caves.entry(cave1).or_insert(vec![]).push(cave2);
        }
    }
    println!("{}", traverse(&caves, 1));
    println!("{}", traverse(&caves, 2));
}
