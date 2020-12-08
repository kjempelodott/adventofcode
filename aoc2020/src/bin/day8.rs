extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;

extern crate parse_display;
use parse_display::FromStr;

#[derive(FromStr,PartialEq,Clone)]
#[display("{} {0}")]
#[display(style="lowercase")]
enum Operation {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

#[derive(Debug)]
enum Exit {
    Ok(isize),
    Error(isize)
}

use self::Operation::*;

fn run(bootcode: &Vec<Operation>) -> Exit {
    let (mut acc, mut ip) = (0, 0);
    let mut visited = vec![];
    while ip < bootcode.len() {
        if visited.contains(&ip) {
            return Exit::Error(acc);
        }
        visited.push(ip);
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
        let orig = bootcode[j].clone();
        match orig {
            Jmp(n) => { bootcode[j] = Nop(n); },
            Nop(n) => { bootcode[j] = Jmp(n); },
            _ => { j += 1; continue; }
        }
        if let Exit::Ok(acc) = run(&bootcode) {
            println!("Part 2: {}", acc);
            break;
        }
        bootcode[j] = orig;
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
