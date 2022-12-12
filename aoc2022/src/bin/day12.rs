extern crate aoc2022;
use aoc2022::read_from_stdin;
use std::collections::{HashMap,BinaryHeap,HashSet};

type Point = (isize, isize);

fn part1(topo: &HashMap<Point,u8>, start: Point, end: Point) -> isize {
    let mut visited = HashSet::new();
    let mut queue: BinaryHeap<(isize,Point)> = BinaryHeap::new();
    queue.push((0, start));
    while let Some((len, cur)) = queue.pop() {
        if !visited.insert(cur) { continue }
        else if cur == end { return - len }
        let curh = *topo.get(&cur).unwrap();
        for (dy,dx) in &[(0,1),(1,0),(0,-1),(-1,0)] {
            let next = (cur.0 + dy, cur.1 + dx);
            if topo.get(&next).map_or(false, |&h| h <= curh + 1) {
                queue.push((len - 1, next));
            }
        }
    }
    0
}

fn part2(topo: &HashMap<Point,u8>, start: Point) -> isize {
    let mut visited = HashSet::new();
    let mut queue: BinaryHeap<(isize,Point)> = BinaryHeap::new();
    queue.push((0, start));
    while let Some((len, cur)) = queue.pop() {
        if !visited.insert(cur) { continue }
        else if topo.get(&cur) == Some(&b'a') { return - len }
        let curh = *topo.get(&cur).unwrap();
        for (dy,dx) in &[(0,1),(1,0),(0,-1),(-1,0)] {
            let next = (cur.0 + dy, cur.1 + dx);
            if topo.get(&next).map_or(false, |&h| h >= curh - 1) {
                queue.push((len - 1, next));
            }
        }
    }
    0
}

fn main() {
    let mut topo = read_from_stdin().lines()
        .enumerate()
        .map(|(y,l)| l.bytes()
             .enumerate()
             .map(move |(x,h)| ((y as isize, x as isize), h)))
        .flatten()
        .collect::<HashMap<_,_>>();

    let start = *topo.iter().find(|(_,h)| **h as char == 'S').unwrap().0;
    let end = *topo.iter().find(|(_,h)| **h as char == 'E').unwrap().0;
    topo.insert(start, 'a' as u8);
    topo.insert(end, 'z' as u8);

    println!("Part 1: {}", part1(&topo, start, end));
    println!("Part 2: {}", part2(&topo, end));
}
