extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;

extern crate parse_display;
use parse_display::FromStr;

#[derive(FromStr,PartialEq)]
#[display("{} {0}")]
#[display(style="lowercase")]
enum Operation {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl Operation {
    fn flip(&mut self) -> bool {
        match self {
            Acc(_) => false,
            Jmp(n) => { *self = Nop(*n); true },
            Nop(n) => { *self = Jmp(*n); true },
        }
    }
}

#[derive(Debug)]
enum Exit {
    Ok(isize),
    Error(isize)
}

use self::Operation::*;

fn run(bootcode: &Vec<Operation>) -> Exit {
    let (mut acc, mut ip) = (0, 0);
    let mut visited = vec![false; bootcode.len()];
    while ip < bootcode.len() {
        if visited[ip] {
            return Exit::Error(acc);
        }
        visited[ip] = true;
        match bootcode[ip] {
            Acc(n) => { acc += n; ip +=1 ; },
            Jmp(n) => { ip = (ip as isize + n) as usize },
            _ => { ip += 1; }
        }
    }
    Exit::Ok(acc)
}

fn fuzz_and_fix(mut bootcode: Vec<Operation>) {
    let mut j = 0;
    loop {
        if bootcode[j].flip() {
            if let Exit::Ok(acc) = run(&bootcode) {
                println!("Part 2: {}", acc);
                break;
            }
            bootcode[j].flip();
        }
        j += 1;
    }
}

fn main() {
    let bootcode: Vec<Operation> = read_from_stdin().lines()
        .map(|l| l.parse().unwrap())
        .collect();
    println!("Part 1: {:?}", run(&bootcode));
    fuzz_and_fix(bootcode);
}
