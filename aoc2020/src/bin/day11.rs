extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;
use adventofcode2020::metric::{Point,Euclidean,Direction};
use Direction::*;

use std::collections::BTreeMap;

#[derive(Clone,Copy,Debug,PartialEq)]
enum Seat {
    Empty,
    Occupied
}

use Seat::*;

fn part1(mut seats: BTreeMap<Point,Seat>) -> usize {
    loop {
        let mut moved = BTreeMap::new();
        for (&p, &s) in seats.iter() {  
            let c = p.adjacent().iter().filter(|&a| seats.get(a) == Some(&Occupied)).count();
            if s == Empty && c == 0 {
                moved.insert(p, Occupied);
            }
            else if s == Occupied && c >= 4 {
                moved.insert(p, Empty);
            }
        }
        if moved.len() == 0 {
            break;
        }
        for (&pos, &seat) in moved.iter() {
            seats.insert(pos, seat);
        }
    }
    seats.values().filter(|&&s| s == Occupied).count()
}

fn part2(mut seats: BTreeMap<Point,Seat>) -> usize {
    let ymax = seats.keys().map(|t| t.y).max().unwrap();
    let xmax = seats.keys().map(|t| t.x).max().unwrap();
    let directions = [Up,
                      Down,
                      Left,
                      Right,
                      Up|Left,
                      Up|Right,
                      Down|Right,
                      Down|Left];

    loop {
        let mut moved = BTreeMap::new();
        for (&p, &s) in seats.iter() {
            let mut c = 0;
            for &d in &directions {
                let mut l = p.clone();
                while l.y >= 0 && l.x >=0 && l.y <= ymax && l.x <= xmax {
                    l.mov(d);
                    if let Some(&state) = seats.get(&l) {
                        if state == Occupied {
                            c += 1;
                        }
                        break
                    }
                }
            }
            if s == Empty && c == 0 {
                moved.insert(p, Occupied);
            }
            else if s == Occupied && c >= 5 {
                moved.insert(p, Empty);
            }
        }
        if moved.len() == 0 {
            break;
        }
        for (&pos, &seat) in moved.iter() {
            seats.insert(pos, seat);
        }
    }
    seats.values().filter(|&&s| s == Occupied).count()
}

fn main() {
    let seats = read_from_stdin().lines()
        .enumerate()
        .map(|(y,l)| l.chars()
             .enumerate()
             .filter(|&(_,c)| c == 'L')
             .map(move |(x,_)| (Point::new(y,x), Empty))
        )
        .flatten()
        .collect::<BTreeMap<Point,Seat>>();
    
    println!("Part 1: {}", part1(seats.clone()));
    println!("Part 2: {}", part2(seats.clone()));
}
