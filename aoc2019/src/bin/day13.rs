#[macro_use]
extern crate adventofcode2019;
use adventofcode2019::{goddamncomputer,read_from_stdin};
use adventofcode2019::goddamncomputer::State;
use adventofcode2019::metric::Point;

use std::collections::HashMap;

#[derive(Debug,PartialEq,Eq,Hash)]
enum TileID {
    Emtpy,
    Wall,
    Block,
    Paddle,
    Ball,
    Score(isize)
}
use TileID::*;

fn draw_tiles(prog: &goddamncomputer::Program) -> HashMap<Point,TileID> {
    prog.run_once(vec![])
        .chunks(3)
        .map(|o| (Point { x: o[0], y: o[1] },
                  match o[2] {
                      0 => Emtpy,
                      1 => Wall,
                      2 => Block,
                      3 => Paddle,
                      4 => Ball,
                      _ => panic!()
                  }))
        .collect()
}

fn main() {
    let mut intcode = numbers!(read_from_stdin() => isize);
    let mut prog = goddamncomputer::Program::new(intcode.clone());
    let mut tiles: HashMap<Point,TileID> = HashMap::new();
    println!("Part 1: {}", draw_tiles(&prog).values().filter(|&t| *t == Block).count());

    intcode[0] = 2;
    prog = goddamncomputer::Program::new(intcode);
    let mut stdin = vec![0];
    while prog.get_state() != State::Halted {
        tiles = prog.run(stdin.clone())
            .chunks(3)
            .map(|o| (Point { x: o[0], y: o[1] },
                      match o[2] {
                          0 => Emtpy,
                          1 => Wall,

                          2 => Block,
                          3 => Paddle,
                          4 => Ball,
                          s => Score(s)
                      }))
            .collect();
        if let Some(ball) = tiles.iter().find(|(_,t)| **t == Ball) {
            if let Some(paddle) = tiles.iter().find(|(_,t)| **t == Paddle) {
                if ball.0.x > paddle.0.x {
                    stdin = vec![1];
                }
                else if ball.0.x < paddle.0.x {
                    stdin = vec![-1];
                }
                else {
                    stdin = vec![0];
                }
            }
        }
    }
    if let Some((_,Score(score))) = tiles.iter().find(|(p,_)| p.x == -1) {
        println!("Part 2: {}", score);
    }
}
