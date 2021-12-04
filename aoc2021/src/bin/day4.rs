#[macro_use]
extern crate aoc2021;
use aoc2021::{into_lines,read_from_stdin};

const SIZE: usize = 5;

struct Cell(u16, bool);
struct Board {
    inner: Vec<Cell>,
    bingo: bool
}


use std::str::FromStr;
use std::num::ParseIntError;
impl FromStr for Cell {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.parse::<u16>()?;
        Ok(Cell(v, false))
    }
}

impl FromIterator<Cell> for Board {
    fn from_iter<I: IntoIterator<Item=Cell>>(iter: I) -> Self {
        let board: Vec<Cell> = iter.into_iter().collect();
        Board { inner: board, bingo: false }
    }
}

impl Board {
    fn mark(&mut self, v: u16) {
        for i in 0..self.inner.len() {
            if self.inner[i].0 == v {
                self.inner[i].1 = true;
                if self.check(i) {
                    self.bingo = true
                }
            }
        }
    }
    fn check(&self, i: usize) -> bool {
        let m = i % SIZE;
        (i-m..i-m+SIZE).all(|j| self.inner[j].1) ||
            (m..SIZE*SIZE).step_by(SIZE).all(|j| self.inner[j].1)
    }
    fn score(&self) -> usize {
        self.inner.iter().filter(|c| !c.1).map(|c| c.0 as usize).sum()
    }
}

fn main() {
    let input = into_lines(read_from_stdin());
    let draws: Vec<_> = numbers![input[0] => u16];
    let mut boards: Vec<Board> = input[1..]
        .chunks(5)
        .map(|b| numbers![b.join(" ") => Cell])
        .collect();
    'draw: for d in draws {
        for i in 0..boards.len() {
            if !boards[i].bingo {
                boards[i].mark(d);
                let board = &boards[i];
                if board.bingo {
                    let nb = boards.iter().filter(|b| b.bingo).count();
                    if nb == 1 {
                        println!("Part 1: {}", board.score() * d as usize);
                    }
                    else if nb == boards.len() {
                        println!("Part 2: {}", board.score() * d as usize);
                        break 'draw;
                    }
                }
            }
        }
    }
}
