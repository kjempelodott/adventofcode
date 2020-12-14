extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;

extern crate parse_display;
use parse_display::{Display,FromStr};

use std::collections::HashMap;

#[derive(Debug,FromStr,Display)]
enum Program {
    #[display("mask = {0}")]
    Mask(String),
    #[display("mem[{0}] = {1}")]
    MemSet(u64,u64)
}

#[derive(Default)]
struct BitMask {
    m: u64,
    c: u64,
    fl: Vec<u64>
}

impl BitMask {
    fn mask(&self, value: u64) -> u64 {
        self.c | self.m & value
    }
    fn update(&mut self, s: &str) {
        self.m = u64::from_str_radix({
            &s.chars()
                .map(|c| if c == 'X' { '1' } else { '0' })
                .collect::<String>()
        },2).unwrap();

        self.c = u64::from_str_radix({
            &s.chars()
                .map(|c| if c == 'X' { '0' } else { c })
                .collect::<String>()
        },2).unwrap();       

        self.fl = (0..36).map(|i| (1 << i) & self.m)
            .filter(|v| *v != 0)
            .collect();
    }
    fn superpositions(&self, addr: u64) -> Vec<u64> {
        let mut space = vec![(self.c | self.m | addr)^self.m];
        for bit in self.fl.iter() {
            let new: Vec<u64> = space.iter()
                .map(|addr| addr | bit)
                .collect();
            space.extend_from_slice(&new);
        }
        space
    }
}

fn main() {
    let mut mask = BitMask::default();
    let mut mem: HashMap<u64,u64> = HashMap::new();
    let mut mem_v2: HashMap<u64,u64> = HashMap::new();

    read_from_stdin().lines()
        .map(|l| l.parse::<Program>().unwrap())
        .for_each(|p| {
            match p {
                Program::Mask(s) => mask.update(&s),
                Program::MemSet(i,n) => {
                    mem.insert(i, mask.mask(n));
                    for a in mask.superpositions(i) {
                        mem_v2.insert(a, n);
                    }
                }
            }
        });
    println!("{}", mem.values().sum::<u64>());
    println!("{}", mem_v2.values().sum::<u64>());
}
