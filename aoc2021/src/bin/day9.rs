extern crate aoc2021;
use aoc2021::read_from_stdin;

use std::collections::{BinaryHeap,HashSet};
type P = (usize,usize);

fn main() {
    let mut w: usize = 0;
    let mut map: Vec<u8> = vec![];
    for line in read_from_stdin().lines() {
        w = line.len();
        map.extend_from_slice(&line.as_bytes());
    }
    let h = map.len()/w;
    let mut risk = 0u32;
    let mut heights: HashSet<P> = HashSet::new();
    let mut lows: HashSet<P> = HashSet::new();
    for j in 0..h {
        for i in 0..w {
            let v = map[j*w + i];
            if v == 57 {
                heights.insert((j,i));
            }
            else if (i == 0 || map[j*w + i - 1] > v) &&
                (i == w-1 || map[j*w + i + 1] > v) &&
                (j == 0 || map[(j-1)*w + i] > v) &&
                (j == h-1 || map[(j+1)*w + i] > v) {
                    lows.insert((j,i));
                    risk += 1 + v as u32 - 48;
                }
        }
    }
    println!("Part 1: {}", risk);

    let mut sizes = Vec::with_capacity(lows.len());
    for &low in lows.iter() {
        let mut size = 0;
        let mut visited = HashSet::new();
        let mut queue = BinaryHeap::new();
        queue.push(low);
        while let Some((j,i)) = queue.pop() {
            if ! visited.insert((j,i)) {
                continue
            }
            size += 1;
            if i != 0   && ! heights.contains(&(j,i-1)) { queue.push((j,i-1)); }
            if i != w-1 && ! heights.contains(&(j,i+1)) { queue.push((j,i+1)); }
            if j != 0   && ! heights.contains(&(j-1,i)) { queue.push((j-1,i)); }
            if j != h-1 && ! heights.contains(&(j+1,i)) { queue.push((j+1,i)); }
        }
        sizes.push(size);
    }
    sizes.sort();
    println!("Part 2: {}", sizes.iter().rev().take(3).fold(1, |s,v| s*v));
}
