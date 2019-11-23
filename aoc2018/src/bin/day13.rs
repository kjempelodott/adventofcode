extern crate adventofcode2018;
use adventofcode2018::read_from_stdin;
use adventofcode2018::metric::{Direction,Point};
use adventofcode2018::metric::Direction::*;
use std::collections::HashMap;

#[derive(Clone,Copy,Debug,Eq,PartialEq,PartialOrd,Ord)]
enum Track {
    ForwardSlash,
    BackwardSlash,
    Intersection
}

#[derive(Clone,Copy,Debug,Eq,PartialEq,PartialOrd,Ord)]
enum Turn {
    Left,
    Straight,
    Right
}

#[derive(Clone,Copy,Debug,Eq,PartialOrd,Ord)]
struct Cart {
    p: Point,
    d: Direction,
    i: Turn
}

impl PartialEq for Cart {
    fn eq(&self, other: &Self) -> bool {
        self.p == other.p
    }
}

impl Cart {
    fn new(y: usize, x: usize, d: Direction) -> Self {
        Cart { p: Point::new(y,x),
               d: d,
               i: Turn::Left }
    }
    fn mov(&mut self) {
        self.p.mov(self.d);
    }
    fn turn(&mut self, t: &Track) {
        self.d = match t {
            Track::ForwardSlash => *[Up, Right, Up, Down, Left, Down].iter()
                .skip_while(|&&d| d != self.d)
                .nth(1).unwrap(),
            Track::BackwardSlash => *[Up, Left, Up, Down, Right, Down].iter()
                .skip_while(|&&d| d != self.d)
                .nth(1).unwrap(),
            Track::Intersection => {
                match self.i {
                    Turn::Left => {
                        self.i = Turn::Straight;
                        *[Right, Up, Left, Down].iter().cycle()
                            .skip_while(|&&d| d != self.d)
                            .nth(1).unwrap()
                    },
                    Turn::Straight => {
                        self.i = Turn::Right;
                        self.d
                    }
                    Turn::Right => {
                        self.i = Turn::Left;
                        *[Right, Up, Left, Down].iter().rev().cycle()
                            .skip_while(|&&d| d != self.d)
                            .nth(1).unwrap()
                    },
                }
            }
        };
        self.mov();
    }
}

fn main() {
    let mut track = HashMap::new();
    let mut carts = vec![];
    for (y,l) in read_from_stdin().lines().enumerate() {
        for (x,c) in l.chars().enumerate() {
            match c {
                '\\' => { track.insert(Point::new(y,x), Track::BackwardSlash); },
                '/' => { track.insert(Point::new(y,x), Track::ForwardSlash); },
                '+' => { track.insert(Point::new(y,x), Track::Intersection); },
                '>' => carts.push(Cart::new(y, x, Right)),
                '^' => carts.push(Cart::new(y, x, Up)),
                '<' => carts.push(Cart::new(y, x, Left)),
                'v' => carts.push(Cart::new(y, x, Down)),
                _ => {}
            }
        }
    } 
    carts.sort_unstable();
    let mut part1_solved = false;
    loop {
        let mut state = carts.clone();
        let mut crashes = vec![];
        for (i, cart) in carts.iter_mut().enumerate() {
            match track.get(&cart.p) {
                Some(track) => cart.turn(&track),
                None => cart.mov()
            }
            if let Some(j) = state.iter().position(|&z| z.p == cart.p) {
                if !part1_solved {
                    part1_solved = true;
                    println!("Part 1: {},{}", cart.p.x, cart.p.y);
                }
                crashes.push(j);
                crashes.push(i);
            }
            state[i] = *cart;
        }
        crashes.sort_unstable();
        crashes.into_iter().rev().for_each(|i| { carts.remove(i); });
        if carts.len() == 1 {
            println!("Part 1: {},{}", carts[0].p.x, carts[0].p.y);
            break
        }
        carts.sort_unstable();
    }
}

