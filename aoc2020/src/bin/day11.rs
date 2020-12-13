extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;

use std::collections::HashMap;

#[derive(Clone,Copy,PartialEq)]
enum Seat {
    Empty,
    Occupied
}

use Seat::*;
type Point = (isize,isize);

fn part1(mut seats: HashMap<Point,Seat>) -> usize {
    let adj = [(0,1),(-1,1),(-1,0),(-1,-1),(0,-1),(1,-1),(1,0),(1,1)];
    let mut moved = true;
    while moved {
        moved = false;
        let mut next = seats.clone();
        for (&p,&s) in seats.iter() {
            let c = adj.iter().filter(|(dy,dx)| seats.get(&(p.0+dy,p.1+dx)) == Some(&Occupied)).count();
            moved |= match c {
                0 if s == Empty => { next.insert(p, Occupied); true }
                4..=8 if s == Occupied => { next.insert(p, Empty); true }
                _ => false
            };
        }
        seats = next;
    }
    seats.values().filter(|v| **v == Occupied).count()
}

fn part2(mut seats: HashMap<Point,Seat>) -> usize {
    let ymax = seats.keys().map(|t| t.0).max().unwrap();
    let xmax = seats.keys().map(|t| t.1).max().unwrap();

    let dir = [(-1,0),(1,0),(0,-1),(0,1),(-1,-1),(-1,1),(1,1),(1,-1)];
    let mut moved = true;
    while moved {
        moved = false;
        let mut next = seats.clone();
        for (&p,&s) in seats.iter() {
            let mut c = 0;
            for &(dy,dx) in &dir {
                let (mut y, mut x) = (p.0 + dy, p.1 + dx);
                while y >= 0 && x >= 0 && y <= ymax && x <= xmax {
                    match seats.get(&(y,x)) {
                        Some(&Occupied) => { c += 1; break },
                        Some(&Empty) => break,
                        None => {}
                    }
                    y += dy;
                    x += dx;
                }
            }
            moved |= match c {
                0 if s == Empty => { next.insert(p, Occupied); true }
                5..=8 if s == Occupied => { next.insert(p, Empty); true }
                _ => false
            };
        }
        seats = next;
    }
    seats.values().filter(|v| **v == Occupied).count()
}

fn main() {
let seats = read_from_stdin().lines()
        .enumerate()
        .flat_map(|(y,l)| l.chars()
                  .enumerate()
                  .filter(|(_,c)| *c == 'L')
                  .map(move |(x,_)| ((y as isize, x as isize), Empty))
        )
        .collect::<HashMap<_,_>>();
    
    println!("Part 1: {}", part1(seats.clone()));
    println!("Part 2: {}", part2(seats));
}
