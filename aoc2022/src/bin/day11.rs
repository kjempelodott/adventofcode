#![feature(int_roundings)]
#[macro_use]
extern crate aoc2022;
use aoc2022::read_from_stdin;
use parse_display::FromStr;

#[derive(Clone,FromStr)]
enum Operation {
    #[display("Operation: new = old + old")]
    Double,
    #[display("Operation: new = old * old")]
    Square,
    #[display("Operation: new = old + {0}")]
    Add(u64),
    #[display("Operation: new = old * {0}")]
    Mul(u64),
}

use Operation::*;

impl Default for Operation {
    fn default() -> Self {
        Add(0)
    }
}

#[derive(Clone,FromStr,Default)]
#[from_str(regex="Monkey [0-9]:\n  Starting items: .+\n  (?P<op>.+)\n  Test: divisible by (?P<modulo>[0-9]+)\n    If true: throw to monkey (?P<cond_0>[0-9]+)\n    If false: throw to monkey (?P<cond_1>[0-9]+)\n?")]
struct Monkey {
    #[from_str(default)]
    items: Vec<u64>,
    op: Operation,
    modulo: u64,
    cond_0: usize, 
    cond_1: usize,
    #[from_str(default)]
    count: usize
}

impl Monkey {
    fn inspect(&mut self) {
        for item in self.items.iter_mut() {
            *item = match self.op {
                Double => *item * 2,
                Square => *item * *item,
                Add(x) => *item + x,
                Mul(x) => *item * x
            }
        }
        self.count += self.items.len();
    }
    fn eval(&mut self, div: u64, m: u64) -> Vec<(u64, usize)> {
        if div == 0 { // Part 2
            self.items.drain(0..).map(|e| (e % m, if e % self.modulo == 0 { self.cond_0 } else { self.cond_1 })).collect()
        }
        else { // Part 1
            self.items.drain(0..).map(|e| e.div_floor(div)).map(|e| (e, if e % self.modulo == 0 { self.cond_0 } else { self.cond_1 })).collect()
        }
    }
}

fn run(mut monkeys: Vec<Monkey>, div: u64, modulo: u64, n: usize) -> usize {
    for _ in 0..n {
        for i in 0..monkeys.len() {
            monkeys[i].inspect();
            let throws = monkeys[i].eval(div, modulo);
            for &(item, j) in throws.iter() {
                monkeys[j].items.push(item);
            }
        }
    }
    monkeys.sort_by_key(|m| m.count);
    let m = monkeys.len() - 1;
    monkeys[m].count * monkeys[m-1].count
}

fn main() {
    let mut monkeys = vec![];
    for input in read_from_stdin().split("\n\n") {
        let mut mt: Monkey = input.parse().unwrap();
        mt.items = numbers!(input.lines().nth(1).unwrap() => u64);
        monkeys.push(mt);
    }
    println!("Part 1: {}", run(monkeys.clone(), 3, 0, 20));
    let modulo = monkeys.iter().fold(1, |x,m| x * m.modulo);
    println!("Part 2: {}", run(monkeys, 0, modulo, 10000));
}
