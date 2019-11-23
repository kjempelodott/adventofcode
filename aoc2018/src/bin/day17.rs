#[macro_use]
extern crate adventofcode2018;
extern crate itertools;
use adventofcode2018::read_from_stdin;
use adventofcode2018::metric::Point;
use adventofcode2018::metric::Direction::*;
use itertools::Itertools;
use std::collections::{HashSet,HashMap,BinaryHeap};

#[derive(Clone,Debug,Eq,PartialEq,PartialOrd,Ord)]
enum Tile {
    FallingWater,
    UnsettledWater,
    SettledWater,
    Clay,
}
use Tile::*;

struct Underground {
    water: HashMap<Point,Tile>,
    clay: HashSet<Point>,
}

impl Underground {
    fn new() -> Self {
        Underground { water: HashMap::new(), clay: HashSet::new() }
    }

    fn add_clay(&mut self, y: isize, x: isize) {
        self.clay.insert(Point::new(y,x));
    }

    fn occupied_by(&self, p: &Point) -> Option<&Tile> {
        if self.clay.contains(&p) {
            return Some(&Clay)
        }
        self.water.get(&p)
    }

    fn get_yminmax(&self) -> (isize, isize) {
        self.clay.iter().map(|&p| p.y).minmax().into_option().unwrap()
    }

    #[allow(dead_code)]
    fn display(&self, xmin: isize, xmax: isize) {
        let yy = self.get_yminmax();
        for y in 0..yy.1+1 {
            println!("{}", (xmin..xmax+1).map(|x| {
                if self.clay.contains(&Point::new(y,x)) {
                    return '#';
                }
                return match self.water.get(&Point::new(y,x)) {
                    None => '.',
                    Some(SettledWater) => '~',
                    _ => '|',
                }
            }).collect::<String>());
        }
    }

    fn drip(&mut self, mut drop: Point) -> Vec<Point> {
        self.water.insert(drop.clone(), FallingWater);
        let next = drop.next_to(Down);

        match self.occupied_by(&next) {
            Some(&Clay)|Some(&SettledWater) => {
                let mut drops = vec![];
                let mut slice = vec![];
                while drops.is_empty() {
                    slice.drain(..).for_each(|p| {
                        self.water.insert(p, SettledWater);
                    });
                    slice.push(drop.clone());
                    for &dir in [Left, Right].into_iter() {
                        let mut w = drop.next_to(dir);
                        while !self.clay.contains(&w) {
                            match self.occupied_by(&w.next_to(Down)) {
                                Some(Clay)|Some(SettledWater) => {
                                    slice.push(w.clone());
                                    w.mov(dir)
                                },
                                None|Some(FallingWater) => {
                                    drops.push(w);
                                    break;
                                },
                                _ => break
                            };
                        }
                    }
                    drop.mov(Up);
                }
                slice.drain(..).for_each(|p| {
                    self.water.insert(p, UnsettledWater);
                });
                drops
            },
            None => vec![next],
            _ => vec![]
        }
    }
}

fn main() {
    let mut underground = Underground::new();
    for l in read_from_stdin().lines() {
        let num = numbers!(l => isize);
        if l.starts_with('x') {
            (num[1]..num[2]+1).for_each(|y| underground.add_clay(y,num[0]));
        }
        else {
            (num[1]..num[2]+1).for_each(|x| underground.add_clay(num[0],x));
        }
    }
    let yy = underground.get_yminmax();

    let mut drops = BinaryHeap::new();
    drops.push(Point::new(0,500));
    while !drops.is_empty() {
        let drop = drops.pop().unwrap();
        if drop.y > yy.1 {
            continue;
        }
        for d in underground.drip(drop).into_iter() {
            drops.push(d);
        }
    }
    println!("Part 1: {}", underground.water.keys()
             .filter(|p| p.y >= yy.0).count());
    println!("Part 2: {}", underground.water.iter()
             .filter(|(p,t)| p.y >= yy.0 && **t == SettledWater).count());
}
