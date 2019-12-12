#[macro_use]
extern crate adventofcode2019;
use adventofcode2019::{goddamncomputer,read_from_stdin};
use adventofcode2019::goddamncomputer::State;
use adventofcode2019::metric::{Point,Direction};
use Direction::*;

use std::collections::HashMap;

extern crate itertools;
use itertools::Itertools;

fn paint(mut prog: goddamncomputer::Program, color: isize) -> HashMap<Point, isize> {
    let mut panels: HashMap<Point, isize> = HashMap::new();
    let mut robot = Point::new(0,0);
    let mut dir = Up;

    panels.insert(robot, color);
    while prog.get_state() != State::Halted {
        let output = prog.run(vec![*panels.get(&robot).unwrap_or(&0)]);
        panels.insert(robot, output[0]);
        dir = if output[1] == 0 { dir.turn_left() } else { dir.turn_right() };
        robot.mov(dir);
    }
    panels
}

fn main() {
    let intcode = numbers!(read_from_stdin() => isize);
    let prog = goddamncomputer::Program::new(intcode);

    println!("Part 1: {}", paint(prog.clone(), 0).len());
    println!("Part 2:");
    let panels = paint(prog, 1);
    let (x0, x1) = panels.keys().map(|p| p.x).minmax().into_option().unwrap();
    let (y0, y1) = panels.keys().map(|p| p.y).minmax().into_option().unwrap();
    for i in y0..y1+1 {
        println!("{}", (x0..x1+1)
                 .map(|j| panels.get(&Point::new(i,j)).unwrap_or(&0))
                 .map(|c| if *c == 0 { ' ' } else { '#' })
                 .collect::<String>())
    }
}
