extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;

fn reduce1(mut rem: &mut Vec<char>) -> usize {
    let mut calc = 0;
    let mut op = '(';
    while let Some(c) = rem.pop() {
        match c {
            ')' => return calc,
            ' ' => { },
            '+' => op = '+',
            '*' => op = '*',
            '(' => match op {
                '+' => calc += reduce1(&mut rem),
                '*' => calc *= reduce1(&mut rem),
                '(' => calc = reduce1(&mut rem),
                _ => unreachable!()
            }
            x => match op {
                '+' => calc += x as usize - 48,
                '*' => calc *= x as usize - 48,
                '(' => calc = x as usize - 48,
                _ => unreachable!()
            },
        }
    }
    calc
}

fn reduce2(mut rem: &mut Vec<char>) -> usize {
    let mut calc = 0;
    while let Some(c) = rem.pop() {
        match c {
            ')' => return calc,
            '+'|' ' => { },
            '*' => return calc * reduce2(&mut rem),
            '(' => calc += reduce2(&mut rem),
            x => calc += x as usize - 48,
        }
    }
    calc
}

fn main() {
    let (p1, p2) = read_from_stdin().lines()
        .fold((0,0), |(mut p1, mut p2),l| {
            p1 += reduce1(&mut l.chars().rev().collect());
            p2 += reduce2(&mut l.chars().rev().collect());
            (p1,p2)
        });
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
