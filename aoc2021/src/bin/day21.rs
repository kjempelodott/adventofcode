#[macro_use]
extern crate aoc2021;
use aoc2021::read_from_stdin;

use std::collections::HashMap;

struct DiracDice {
    score: u64,
    rolls_mod: u64,
    rolls_div: u64
}

impl DiracDice {
    const DIV: u64 = 100;
    fn new() -> Self {
        DiracDice {
            score: 0,
            rolls_mod: Self::DIV,
            rolls_div: 0
        }
    }
    fn roll(&mut self) -> u64 {
        if self.rolls_mod > (Self::DIV - 3) {
            self.score = 3 * (2 - (Self::DIV - self.rolls_mod));
            self.rolls_mod = (self.rolls_mod % (Self::DIV - 2)) + 1;
            self.rolls_div += 1;
        }
        else {
            self.score += 9;
            self.rolls_mod += 3;
        }
        self.score
    }
    fn get_rolls(&self) -> u64 {
        Self::DIV * (self.rolls_div - 1) + self.rolls_mod
    }
}

type QPlayer = (u64,u64); 
const QROLLS: [(u64,u64);7] = [(3,1),(4,3),(5,6),(6,7),(7,6),(8,3),(9,1)];

fn main() {
    let p: Vec<_> = numbers!(read_from_stdin() => u64);
    let (mut p1, mut p2) = (p[1] - 1, p[3] - 1);
    let (mut s1, mut s2) = (0, 0);
    let mut dirac_dice = DiracDice::new();
    loop {
        p1 = (p1 + dirac_dice.roll()) % 10;
        s1 += p1 + 1;
        if s1 >= 1000 {
            println!("Part 1: {}", dirac_dice.get_rolls() * s2);
            break
        }

        p2 = (p2 + dirac_dice.roll()) % 10;
        s2 += p2 + 1;
        if s2 >= 1000 {
            println!("Part 1: {}", dirac_dice.get_rolls() * s1);
            break
        }
    }

    let mut games: HashMap<(QPlayer,QPlayer),u64> = HashMap::new();
    games.insert(((p[1]-1,0),(p[3]-1,0)),1);
    let (mut p1_wins, mut p2_wins) = (0, 0);
    while ! games.is_empty() {
        let mut next_games = HashMap::new();
        for ((p1,p2), entropy) in games.iter() {
            for (r,n) in QROLLS.iter() {
                let pos = (p1.0 + r) % 10;
                let score = p1.1 + pos + 1;
                if score >= 21 {
                    p1_wins += entropy * n;
                    continue;
                }
                let player1 = (pos, score);
                for (r,m) in QROLLS.iter() {
                    let pos = (p2.0 + r) % 10;
                    let score = p2.1 + pos + 1;
                    if score >= 21 {
                        p2_wins += entropy * n * m;
                        continue;
                    }
                    let player2 = (pos, score);
                    *next_games.entry((player1,player2)).or_insert(0) += entropy * n * m;
                }
            }
        }
        games = next_games;
    }
    println!("Part 2: {}", std::cmp::max(p1_wins, p2_wins));
}
