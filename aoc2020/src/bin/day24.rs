extern crate adventofcode2020;                                                                                                                                                                                      
use adventofcode2020::read_from_stdin;

extern crate regex;
use regex::Regex;

use std::collections::HashSet;
use std::hash::Hash;

#[derive(Hash,PartialEq,Eq,Clone,Copy)]
struct Hexagon(i32,i32);

trait Space: Clone + Copy + PartialEq + Eq + Hash {
    fn neighbors(&self) -> Vec<Self> where Self: Sized;
    fn mov(&self, &str) -> Self;
}

impl Space for Hexagon {
    fn mov(&self, dir: &str) -> Self {
        let d = match dir {
            "e"  => ( 0, 2),
            "w"  => ( 0,-2),
            "nw" => ( 1,-1),
            "sw" => (-1,-1),
            "ne" => ( 1, 1),
            "se" => (-1, 1),
            _ => unreachable!()
        };
        Hexagon(self.0+d.0, self.1+d.1)
    }
    
    fn neighbors(&self) -> Vec<Self> {
        [( 0, 2),
         ( 0,-2),
         ( 1,-1),
         (-1,-1),
         ( 1, 1),
         (-1, 1)]
            .iter()
            .map(|d| Hexagon(self.0+d.0,self.1+d.1))
            .collect()
    }
}

fn tick<T: Space>(hexagons: &mut HashSet<T>) {
    let mut tmp = hexagons.clone();
    for h in hexagons.iter() {
        // Check all black tiles
        let n = h.neighbors().iter().filter(|x| hexagons.contains(&x)).count();
        match n {
            1|2 => {},
            _ => { tmp.remove(&h); }
        };
        // Check all white neighbors
        for &nb in h.neighbors().iter().filter(|x| !hexagons.contains(&x)) {
            if nb.neighbors().iter().filter(|x| hexagons.contains(&x)).count() == 2 {
                tmp.insert(nb);
            }
        }
    }
    *hexagons = tmp;
}

fn main() {
    let re = Regex::new(r"nw|ne|sw|se|e|w").unwrap();
    let mut black = HashSet::new();
    for l in read_from_stdin().lines() {
        let p = re.captures_iter(&l)
            .fold(Hexagon(0,0), |h, s| h.mov(&s[0]));
        if !black.insert(p) {
            black.remove(&p);
        }
    }
    println!("Part 1: {}", black.len());

    (0..100).for_each(|_| tick(&mut black));
    println!("Part 2: {}", black.len());
}
