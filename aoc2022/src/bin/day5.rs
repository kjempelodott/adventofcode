#[macro_use]
extern crate aoc2022;
use aoc2022::read_from_stdin;

trait Crane {
    fn mov(n: usize, stacks: &mut Vec<Vec<char>>, a: usize, b: usize);
}

struct CrateMover9000;
struct CrateMover9001;

impl Crane for CrateMover9000 {
    fn mov(n: usize, stacks: &mut Vec<Vec<char>>, a: usize, b: usize) {
        for _ in 0..n {
            if let Some(e) = stacks[a].pop() {
                stacks[b].push(e);
            }
            else { break }
        }
    }
}

impl Crane for CrateMover9001 {
    fn mov(n: usize, stacks: &mut Vec<Vec<char>>, a: usize, b: usize) {
        let split = if n > stacks[a].len() { 0 } else { stacks[a].len() - n };
        let mut boxes = stacks[a].split_off(split);
        stacks[b].append(&mut boxes);
    }
}

fn main() {
    let input = read_from_stdin();
    let (stack_input, instructions) = input.split_once("\n\n").unwrap();
    let mut lines: Vec<&str> = stack_input.lines().collect();
    let z: Vec<_> = numbers!(lines.pop().unwrap() => usize);
    let mut stacks: Vec<Vec<char>> = (0..z.len())
        .map(|i| lines.iter()
             .rev()
             .map(|l| l.chars().nth(1 + 4*i).unwrap())
             .take_while(|c| c.is_alphabetic())
             .collect())
        .collect();
    let mut stacks_2 = stacks.to_vec();

    for line in instructions.lines() {
        let inst: Vec<usize> = numbers!(line => usize);
        let (n, a, b) = (inst[0], inst[1] - 1, inst[2] - 1);
        CrateMover9000::mov(n, &mut stacks, a, b);
        CrateMover9001::mov(n, &mut stacks_2, a, b);
    }

    println!("Part 1: {}", stacks.iter()
             .filter_map(|stack| stack.iter().last())
             .collect::<String>()
    );
    println!("Part 2: {}", stacks_2.iter()
             .filter_map(|stack| stack.iter().last())
             .collect::<String>()
    );
}
