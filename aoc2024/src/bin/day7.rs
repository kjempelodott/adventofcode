#![feature(strict_overflow_ops)]
use aoc2024::{numbers,read_from_stdin};

fn concat(a: usize, b: usize) -> usize {
    b + a * 10usize.strict_pow(1 + b.ilog10())
}

fn solve1(answer: usize, cur: usize, rem: &[usize]) -> bool {
    if rem.len() == 0 {
        return cur == answer
    }
    solve1(answer, cur + rem[0], &rem[1..]) || solve1(answer, cur * rem[0], &rem[1..])
}

fn solve2(answer: usize, cur: usize, rem: &[usize]) -> bool {
    if rem.len() == 0 {
        return cur == answer
    }
    solve2(answer, cur + rem[0], &rem[1..]) || solve2(answer, cur * rem[0], &rem[1..]) || solve2(answer, concat(cur, rem[0]), &rem[1..])
}

fn main() {
    let mut sum1 = 0; 
    let mut sum2 = 0;
    for line in read_from_stdin().lines() {
        let eq: Vec<usize> = numbers!(line => usize);
        if solve1(eq[0], eq[1], &eq[2..]) {
            sum1 += eq[0];
        }
        else if solve2(eq[0], eq[1], &eq[2..]) {
            sum2 += eq[0];
        }
    }
    println!("Part 1: {}", sum1);
    println!("Part 2: {}", sum1 + sum2);
}
