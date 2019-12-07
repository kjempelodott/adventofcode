extern crate adventofcode2019;
use adventofcode2019::{into_lines,read_from_stdin};
use adventofcode2019::metric::{Point,Direction};

use std::collections::{HashSet,HashMap};

fn main() {
    let input = into_lines(read_from_stdin());
    let wire1 = make_wire(&input[0]);
    let wire2 = make_wire(&input[1]);
    let wire1_points = wire1.keys().into_iter().collect::<HashSet<&Point>>();
    let wire2_points = wire2.keys().into_iter().collect::<HashSet<&Point>>();
    let crossings = wire1_points.intersection(&wire2_points);

    let p = crossings.clone().into_iter()
        .min_by_key(|p| p.x.abs() + p.y.abs())
        .unwrap();
    println!("Part 1: {}", p.x.abs() + p.y.abs());

    let q = crossings.into_iter()
        .min_by_key(|p| wire1.get(p).unwrap() + wire2.get(p).unwrap())
        .unwrap();    
    println!("Part 2: {}", wire1.get(q).unwrap() + wire2.get(q).unwrap());
}

fn make_wire(input: &String) -> HashMap<Point,usize> {
    let mut p = Point::new(0,0);
    let mut steps = 0;
    let mut wire = HashMap::new();
    for i in input.split(",") {
        let (d,n) = i.split_at(1);
        let dir = match d {
            "L" => Direction::Left,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "U" => Direction::Up,
            _ => panic!()
        };
        for _ in 0..n.parse::<usize>().unwrap() {
            steps += 1;
            p.mov(dir);
            wire.entry(p).or_insert(steps);
        }
    }
    wire
}
