#[macro_use]
extern crate adventofcode2018;
extern crate itertools;
extern crate counter;
use adventofcode2018::read_from_stdin;
use itertools::Itertools;
use counter::Counter;
use std::collections::{HashSet,HashMap};

#[derive(Debug)]
struct Pos(isize, isize, isize);
#[derive(Debug)]
struct Bot {
    pos: Pos,
    range: isize
}

impl Pos {
    fn dist(&self, o: &Pos) -> isize {
        (self.0-o.0).abs() + (self.1-o.1).abs() + (self.2-o.2).abs()
    }
}

impl Bot {
    fn in_range(&self, other: &Pos) -> bool {
        self.dist(&other) <= self.range
    }
    fn dist(&self, o: &Pos) -> isize {
        self.pos.dist(&o)
    }
}

fn main() {
    let input: Vec<String> = split_into!(read_from_stdin());
    let mut bots: Vec<Bot> = input.iter()
        .map(|l| {
            let p = numbers!(l => isize);
            Bot { pos: (Pos(p[0], p[1], p[2])), range: p[3] }
        })
        .collect();

    let max_r_bot = bots.iter().max_by_key(|b| b.range).unwrap();
    let n_in_range = bots.iter()
        .filter(|b| max_r_bot.in_range(&b.pos))
        .count();

    println!("Part 1: {}", n_in_range);
    
    let mut x = bots.iter().map(|b| b.pos.0).minmax().into_option().unwrap();
    let mut y = bots.iter().map(|b| b.pos.1).minmax().into_option().unwrap();
    let mut z = bots.iter().map(|b| b.pos.2).minmax().into_option().unwrap();
    
    let origo = Pos(0,0,0);
    let mut box_size = 2usize.pow(1);
    let mut best = 0;
    while box_size > 1 {
        let d = box_size as isize;
        let mut best_md = 0;
        for i in (x.0..x.1).step_by(box_size) {
            for j in (y.0..y.1).step_by(box_size) {
                for k in (z.0..z.1).step_by(box_size) {
                    let this = Pos(i,j,k);
                    let in_range = bots.iter()
                        .filter(|b| b.dist(&this) < b.range + d)
                        .count();
                    if in_range >= best {
                        let md = origo.dist(&this);
                        if in_range > best || md < best_md {
                            best = in_range;
                            best_md = origo.dist(&this);
                            x = (i-d,i+d);
                            y = (j-d,j+d);
                            z = (k-d,k+d);
                        }
                    }
                }
                println!("{}", box_size);
                println!("{}", best);
                println!("{}", best_md);

            }
        }
        box_size = box_size >> 1;
    }    
}
