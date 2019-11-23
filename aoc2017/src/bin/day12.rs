// Keywords: travelling salesman, regions

#[macro_use]
extern crate adventofcode2017;
use std::collections::{BinaryHeap, BTreeSet};

fn solve(input: &str) -> (usize, usize) {
    let programs: Vec<BTreeSet<usize>> = input.lines()
        .map(|l| l.split_whitespace())
        .map(|l| l.skip(2)
             .map(|n| ja![n.trim_matches(',').parse()])
             .collect()
        ).collect();

    let mut groups: Vec<BTreeSet<usize>> = vec![];
    let mut seen: BTreeSet<usize> = BTreeSet::new();
    for (p, others) in programs.iter().enumerate() {
        if seen.contains(&p) {
            continue
        }
        let mut queue: BinaryHeap<&usize> = others.iter().collect();
        let mut group: BTreeSet<usize> = BTreeSet::new();
        group.insert(p);
        while let Some(p) = queue.pop() {
            if seen.contains(&p) {
                continue
            }
            group.insert(*p);
            seen.insert(*p);
            let mut others: BinaryHeap<&usize> = programs[*p].iter().collect();
            queue.append(&mut others);
        }
        groups.push(group);
    }
    (groups[0].len(), groups.len())
}

fn main() {
    let (group0, ngroups) = solve(include_str!("../../res/day12"));
    println!("Part 1: {}", group0);
    println!("Part 2: {}", ngroups);
}
