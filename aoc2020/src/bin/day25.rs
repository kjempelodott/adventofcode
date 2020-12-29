#[macro_use]
extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;

const DIV: usize = 20201227;

trait PublicKey {
    const SUBJECT: usize;
    fn crack_rounds(&mut self);
    fn encryption_key(&self, other: &Self) -> usize;
}

struct Chip {
    public_key: usize,
    rounds: Option<usize>
}

impl PublicKey for Chip {
    const SUBJECT: usize = 7;
    fn crack_rounds(&mut self) {
        let mut n = 1;
        let mut rounds = 0;
        while n != self.public_key {
            n = (Chip::SUBJECT * n) % DIV;
            rounds += 1
        }
        self.rounds = Some(rounds);
    }
    fn encryption_key(&self, other: &Chip) -> usize {
        let mut n = 1;
        for _ in 0..other.rounds.unwrap() {
            n = (self.public_key * n) % DIV;
        }
        n
    }
}

fn main() {
    let nums: Vec<usize> = numbers!(read_from_stdin() => usize);
    let mut door = Chip { public_key: nums[0], rounds: None };
    let card = Chip { public_key: nums[1], rounds: None };
    door.crack_rounds();
    println!("Part 1: {}", card.encryption_key(&door));
}
