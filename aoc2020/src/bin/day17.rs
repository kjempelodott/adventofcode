extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;

#[macro_use]
extern crate itertools;

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone,Copy,Debug,PartialEq)]
enum Cube {
    Active,
    Inactive
}
use Cube::*;

#[derive(Hash,PartialEq,Eq,Clone,Copy)]
struct Point(isize,isize,isize);
#[derive(Hash,PartialEq,Eq,Clone,Copy)]
struct HyperPoint(isize,isize,isize,isize);

trait Space: Clone + Copy + PartialEq + Eq + Hash {
    fn neighbors(&self) -> Vec<Self> where Self: Sized;
}

impl Space for Point {
    fn neighbors(&self) -> Vec<Self> {
        iproduct!(-1..2,-1..2,-1..2)
            .filter(|d| *d != (0,0,0))
            .map(|d| Point(self.0+d.0,self.1+d.1,self.2+d.2))
            .collect()
    }
}

impl Space for HyperPoint {
    fn neighbors(&self) -> Vec<Self> {
        iproduct!(-1..2,-1..2,-1..2,-1..2)
            .filter(|d| *d != (0,0,0,0))
            .map(|d| HyperPoint(self.0+d.0,self.1+d.1,self.2+d.2,self.3+d.3))
            .collect()
    }
}

fn tick<T: Space>(cubes: &mut HashMap<T,Cube>) {
    let mut tmp = cubes.clone();
    for (p,_) in tmp.iter().filter(|(_,&s)| s == Active) {
        for &nb in p.neighbors().iter() {
            cubes.entry(nb).or_insert(Inactive);
        }
    }
    tmp = cubes.clone();
    for (&p,&s) in cubes.iter() {
        let c = p.neighbors().iter().filter(|nb| cubes.get(&nb) == Some(&Active)).count();
        match s {
            Active if c != 2 && c != 3 => { tmp.insert(p, Inactive); },
            Inactive if c == 3 => { tmp.insert(p, Active); },
            _ => {}
        };
    }
    *cubes = tmp;
}

fn main() {
    let mut cubes = read_from_stdin().lines()
        .enumerate()
        .flat_map(|(y,l)| l.chars()
                  .enumerate()
                  .filter(|(_,c)| *c == '#')
                  .map(move |(x,_)| (Point(0, y as isize, x as isize), Active))
        )
        .collect::<HashMap<Point,Cube>>();
    let mut hypercubes = cubes.iter()
        .map(|(p,c)| (HyperPoint(0isize, p.0, p.1, p.2), *c))
        .collect::<HashMap<HyperPoint,Cube>>();

    (0..6).for_each(|_| {
        tick(&mut cubes);
        tick(&mut hypercubes);
    });

    println!("Part 1: {}", cubes.values().filter(|v| **v == Active).count());
    println!("Part 2: {}", hypercubes.values().filter(|v| **v == Active).count());
}
