extern crate aoc2021;
use aoc2021::read_from_stdin;

use std::collections::{BinaryHeap,HashSet};

fn traverse(w: usize, h: usize, map: &Vec<u8>) -> i64 {
    let mut queue = BinaryHeap::from([(map[0] as i64,0usize,0usize)]);
    let mut visited = HashSet::new();
    while let Some((mut r,j,i)) = queue.pop() {
        if ! visited.insert((j,i)) {
            continue
        }
        r -= map[j*w + i] as i64;
        if j == h-1 && i == w-1 {
            return -r;
        }
        if i != 0   { queue.push((r,j,i-1)); }
        if i != w-1 { queue.push((r,j,i+1)); }
        if j != 0   { queue.push((r,j-1,i)); }
        if j != h-1 { queue.push((r,j+1,i)); }
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
    println!("Part 1: {}", traverse(w, h, &map));

    map = input.lines()
        .flat_map(|l| l.bytes()
                  .cycle()
                  .enumerate()
                  .map(|(i,b)| b - 48 + (i / w) as u8)
                  .map(|b| if b > 9 { b - 9 } else { b })
                  .take(5*w)
        )
        .cycle().enumerate()
        .map(|(i,t)| t + (i / (5*w*h)) as u8)
        .map(|b| if b > 9 { b - 9 } else { b })
        .take(5*5*w*h)
        .collect();
    println!("Part 2: {}", traverse(5*w, 5*h, &map));
}
