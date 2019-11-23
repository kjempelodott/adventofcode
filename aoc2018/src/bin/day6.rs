#[macro_use]
extern crate adventofcode2018;
extern crate itertools;
extern crate counter;
use adventofcode2018::read_from_stdin;
use adventofcode2018::metric::{Point,Manhattan};
use std::collections::HashSet;
use itertools::Itertools;
use counter::Counter;

fn main() {
    let mut locs: Vec<Point> = numbers!(read_from_stdin() => isize).iter()
        .tuples()
        .map(|(x,y)| Point::new(*y,*x))
        .collect();

    let xm = locs.iter().map(|p| p.x).minmax().into_option().unwrap();
    let ym = locs.iter().map(|p| p.y).minmax().into_option().unwrap();

    let infinite: HashSet<Point> =
               (xm.0..xm.1).map(|x| (x, ym.0))
        .chain((xm.0..xm.1).map(|x| (x, ym.1)))
        .chain((ym.0..ym.1).map(|y| (xm.0, y)))
        .chain((ym.0..ym.1).map(|y| (xm.1, y)))
        .filter_map(|(x,y)| Point::new(y,x).nearest(&mut locs))
        .collect();

    let finite: HashSet<Point> = locs.clone().into_iter()
        .filter(|&p| !infinite.contains(&p))
        .collect();

    // Find largest finite region in Voronoi diagram
    println!("Part 1: {}", (xm.0..xm.1).cartesian_product(ym.0..ym.1)
             .filter_map(|(x,y)| Point::new(y,x).nearest(&mut locs))
             .filter(|&p| finite.contains(&p))
             .collect::<Counter<Point>>()
             .most_common()[0].1);
    println!("Part 2: {}", (xm.0..xm.1).cartesian_product(ym.0..ym.1)
             .filter(|(x,y)| locs.iter()
                     .map(|&p| p.distance(&Point::new(*y,*x)))
                     .sum::<usize>() < 10000)
             .count());
}
