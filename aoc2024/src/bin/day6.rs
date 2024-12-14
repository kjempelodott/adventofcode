use std::collections::BTreeSet;
use aoc2024::read_from_stdin;

type C = (isize, isize);

fn solve(mut guard: C, crates: BTreeSet<C>, b: C) -> Option<BTreeSet<C>> {
    let dd = [(-1,0), (0,1), (1,0), (0,-1)];
    let mut rot = dd.iter().cycle();
    let mut dir = rot.next().unwrap();

    let mut states = BTreeSet::new();
    let mut visited = BTreeSet::new();
    while guard.0 < b.0 && guard.0 >= 0 && guard.1 < b.1 && guard.1 >= 0 {
        if states.insert((guard, dir)) == false {
            return None
        }
        visited.insert(guard);
        let mut next = (guard.0 + dir.0, guard.1 + dir.1);
        while crates.contains(&next) {
            dir = rot.next().unwrap();
            next = (guard.0 + dir.0, guard.1 + dir.1);
        }
        guard.0 += dir.0;
        guard.1 += dir.1;
    }
    Some(visited)
}

fn main() {
    let input = read_from_stdin();
    let mut guard = (0,0);
    let max_x = input.lines().next().unwrap().len() as isize;
    let max_y = input.lines().count() as isize;
    let mut crates = BTreeSet::new();

    for (j,line) in input.lines().enumerate() {
        for (i,c) in line.chars().enumerate() {
            match c {
                '#' => {
                    crates.insert((j as isize, i as isize));
                }
                '^' => {
                    guard = (j as isize, i as isize);
                },
                _ => {}
            }
        }
    }

    let bounds = (max_y, max_x);
    let mut visited = solve(guard.clone(), crates.clone(), bounds).unwrap();
    println!("Part 1: {}", visited.len());

    let mut loops = 0;
    visited.remove(&guard);
    for v in visited.iter() {
        let mut crates_part2 = crates.clone();
        crates_part2.insert(*v);
        if solve(guard.clone(), crates_part2, bounds).is_none() {
            loops += 1;
        }
    }
    println!("Part 2: {}", loops);
}
