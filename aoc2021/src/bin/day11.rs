extern crate aoc2021;
use aoc2021::read_from_stdin;

use std::collections::{BinaryHeap,HashMap,HashSet};
type P = (isize,isize);

fn flashdance(octi: &mut HashMap<P,u8>) -> usize {
    octi.values_mut().for_each(|charge| { *charge += 1; });
    let mut charged: BinaryHeap<P> = octi.iter()
        .filter(|(_,c)| **c == 10)
        .map(|(&p,_)| p)
        .collect();
    let mut depleted: HashSet<P> = HashSet::new();
    while let Some((j,i)) = charged.pop() {
        if ! depleted.insert((j,i)) {
            continue;
        }            
        for (dj,di) in [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)] {
            if let Some(c) = octi.get_mut(&(j+dj,i+di)) {
                *c += 1;
                if *c == 10 {
                    charged.push((j+dj,i+di));
                }
            }
        }
    }
    depleted.iter().for_each(|p| { *octi.get_mut(&p).unwrap() = 0; });
    depleted.len()
}

fn main() {
    let mut octi: HashMap<P,u8> = read_from_stdin().lines()
        .zip(0..10)
        .flat_map(|(line,j)| line.bytes()
                  .zip(0..10)
                  .map(move |(c,i)| ((j,i), c - 48))
        )
        .collect();

    println!("Part 1: {}", (0..100).map(|_| flashdance(&mut octi)).sum::<usize>());
    println!("Part 2: {}", (101..).skip_while(|_| flashdance(&mut octi) != 100).next().unwrap());
}
