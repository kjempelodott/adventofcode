extern crate aoc2021;
use aoc2021::read_from_stdin;

use std::collections::{BinaryHeap,HashSet};

fn traverse(s: usize, w: usize, h: usize, map: &Vec<u8>) -> i64 {
    let mut queue = BinaryHeap::from([(map[0] as i64,0usize,0usize)]);
    let mut visited = HashSet::new();
    while let Some((mut r,j,i)) = queue.pop() {
        if ! visited.insert((j,i)) {
            continue
        }
        let z = map[(j % h)*w + (i % w)] as i64 + (i / w) as i64 + (j / h) as i64;
        r -= if z > 9 { z - 9} else { z };
        if j == s*h-1 && i == s*w-1 {
            return -r;
        }
        if i != 0     { queue.push((r,j,i-1)); }
        if i != s*w-1 { queue.push((r,j,i+1)); }
        if j != 0     { queue.push((r,j-1,i)); }
        if j != s*h-1 { queue.push((r,j+1,i)); }
    }
    unreachable!();
}

fn main() {
    let input = read_from_stdin();
    let mut w: usize = 0;
    let mut map: Vec<u8> = vec![];
    for line in input.lines() {
        w = line.len();
        map.append(&mut line.bytes().map(|b| b - 48).collect());
    }
    let h = map.len()/w;
    println!("Part 1: {}", traverse(1, w, h, &map));
    println!("Part 2: {}", traverse(5, w, h, &map));
}
