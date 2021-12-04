#![feature(drain_filter)]

#[macro_use]
extern crate aoc2021;
use aoc2021::{into_lines,read_from_stdin};

struct Board {
    inner: Vec<u16>,
    mask: u32
}

const BINGOS: [u32;10] = [
    0b1111100000000000000000000,
    0b0000011111000000000000000,
    0b0000000000111110000000000,
    0b0000000000000001111100000,
    0b0000000000000000000011111,
    0b1000010000100001000010000,
    0b0100001000010000100001000,
    0b0010000100001000010000100,
    0b0001000010000100001000010,
    0b0000100001000010000100001,
];

impl FromIterator<u16> for Board {
    fn from_iter<I: IntoIterator<Item=u16>>(iter: I) -> Self {
        let board: Vec<u16> = iter.into_iter().collect();
        Board { inner: board, mask: 0 }
    }
}

impl Board {
    fn mark(&mut self, v: u16) {
        self.mask |= (0..25).filter(|&i| self.inner[i] == v).map(|i| 1 << i).sum::<u32>();
    }
    fn check(&self) -> bool {
        BINGOS.iter().any(|&m| m & self.mask == m)
    }
    fn score(&self) -> usize {
        (0..25).filter(|i| (1 << i) & self.mask == 0).map(|i| self.inner[i] as usize).sum()
    }
}

fn main() {
    let input = into_lines(read_from_stdin());
    let draws: Vec<_> = numbers![input[0] => u16];
    let mut boards: Vec<Board> = input[1..]
        .chunks(5)
        .map(|b| numbers![b.join(" ") => u16])
        .collect::<Vec<Board>>();

    let n_boards = boards.len();
    for d in draws {
        boards.iter_mut().for_each(|b| b.mark(d));
        let bingos = boards.drain_filter(|b| b.check()).collect::<Vec<Board>>();
        if bingos.len() > 0 {
            if n_boards - boards.len() == 1 {
                println!("Part 1: {}", bingos[0].score() * d as usize);
            }
            else if boards.is_empty() {
                println!("Part 2: {}", bingos[0].score() * d as usize);
                break
            }
        }
    }
}
