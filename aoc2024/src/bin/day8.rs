use itertools::Itertools;
use std::collections::{HashSet,HashMap};
use aoc2024::read_from_stdin;

type C = (isize, isize);
type Frequency = char; 

fn main() {
    let input = read_from_stdin();
    let mut map: HashMap<Frequency, Vec<C>> = HashMap::new();

    let max_x = input.lines().next().unwrap().len() as isize;
    let max_y = input.lines().count() as isize;

    for (j,line) in input.lines().enumerate() {
        for (i,c) in line.chars().enumerate() {
            if c == '.' { continue }
            map.entry(c).or_default().push((j as isize,i as isize));
        }
    }

    let mut antinodes = HashSet::new();
    let mut antinodes2 = HashSet::new();
    let validate = |a: C| a.0 >= 0 && a.0 < max_x && a.1 >= 0 && a.1 < max_y;

    for pos in map.values() {
        for pair in pos.into_iter().combinations(2) {
            let (v, w) = (pair[0], pair[1]);
            antinodes2.insert(*v);
            antinodes2.insert(*w);
            let dv = (v.0 - w.0, v.1 - w.1);
            let mut a1 = (v.0 + dv.0, v.1 + dv.1);
            let mut a2 = (w.0 - dv.0, w.1 - dv.1);
            if validate(a1) {
                antinodes.insert(a1);
                while validate(a1) {
                    antinodes2.insert(a1);
                    a1.0 += dv.0;
                    a1.1 += dv.1;
                }
            }
            while validate(a2) {
                antinodes.insert(a2);
                while validate(a2) {
                    antinodes2.insert(a2);
                    a2.0 -= dv.0;
                    a2.1 -= dv.1;
                }
            }
        }
    }
    println!("Part 1: {}", antinodes.len());
    println!("Part 2: {}", antinodes2.len());
}
