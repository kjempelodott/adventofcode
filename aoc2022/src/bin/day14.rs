#[macro_use]
extern crate aoc2022;
use aoc2022::read_from_stdin;
use std::collections::HashSet;

type Point = (isize, isize);

fn flow(solid: &mut HashSet<Point>, mut pos: Point, ymax: isize) -> Point {
    'outer: loop {
        for p in [(pos.0 + 1, pos.1), (pos.0 + 1, pos.1 - 1), (pos.0 + 1, pos.1 + 1)] {
            if p.0 == ymax + 2 {
                continue
            }
            if !solid.contains(&p) {
                pos = p;
                continue 'outer;
            }
        }
        break;
    }
    solid.insert(pos);
    pos
}

fn map_line(x0: isize, y0: isize, x1: isize, y1: isize) -> Box<dyn Iterator<Item = Point>> {
    if x0 == x1 {
        Box::new(if y0 < y1 { y0..y1+1 } else { y1..y0+1 }
                 .map(move |y| (y, x0.clone())))
    }
    else {
        Box::new(if x0 < x1 { x0..x1+1 } else { x1..x0+1 }
                 .map(move |x| (y0.clone(), x)))
    }
}

fn main() {
    let mut solid = read_from_stdin().lines()
        .map(|l| {
            let num: Vec<isize> = numbers!(l => isize);
            num.windows(4).step_by(2).flat_map(|w| map_line(w[0], w[1], w[2], w[3])).collect::<Vec<Point>>()
        })
        .flatten()
        .collect::<HashSet<Point>>();

    let ymax = solid.iter().max().unwrap().0;
    let mut sum = (0..).take_while(|_| flow(&mut solid, (0,500), ymax).0 <= ymax).count();
    println!("Part 1: {sum}");
    sum += (0..).take_while(|_| flow(&mut solid, (0,500), ymax) != (0,500)).count() + 2;
    println!("Part 2: {sum}");
}
