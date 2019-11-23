#![feature(drain_filter)]

#[macro_use]
extern crate adventofcode2018;
use adventofcode2018::read_from_stdin;
use adventofcode2018::metric::{Point4D,Manhattan};
use std::collections::HashSet;

fn main() {
    let mut points: Vec<Point4D> = read_from_stdin().lines()
        .map(|l| {
            let n = numbers!(l => isize);
            Point4D::new(n[0],n[1],n[2],n[3])
        })
        .collect();

    let mut constellations = vec![];
    while !points.is_empty() {
        let p = points.pop().unwrap();
        let mut c = HashSet::new();
        c.insert(p);
        while points.drain_filter(|q| {
            if c.iter().any(|r| r.distance(&q) < 4) {
                c.insert(*q);
                return true
            }
            false
        }).count() > 0 {}
        constellations.push(c);
    }
    println!("Part 1: {:?}", constellations.len());
}
