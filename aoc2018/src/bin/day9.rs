#[macro_use]
extern crate adventofcode2018;
use adventofcode2018::read_from_stdin;
use std::collections::VecDeque;

fn rotate_anti_clockwise(buffer: &mut VecDeque<usize>, n: usize) {
    (0..n).for_each(|_| {
        let last = buffer.pop_back().unwrap();
        buffer.push_front(last);
    })
}

fn rotate_clockwise(buffer: &mut VecDeque<usize>, n: usize) {
    (0..n).for_each(|_| {
        let first = buffer.pop_front().unwrap();
        buffer.push_back(first);
    })
}

fn solve(players: usize, marbles: usize) -> usize {
    let mut score = vec![0; players];
    // Circular, increasing buffer
    let mut circle = VecDeque::with_capacity(marbles);
    circle.push_back(0);
    (1..marbles+1).for_each(|m| {
        if m % 23 == 0 {
            rotate_anti_clockwise(&mut circle, 7);
            score[m % players] += m + circle.pop_front().unwrap();
        } else {
            rotate_clockwise(&mut circle, 2);
            circle.push_front(m);
        }
    });
    *(score.iter().max().unwrap())
}

fn main() {
    let input = numbers!(read_from_stdin() => usize);
    let (players, marbles) = (input[0], input[1]);
    println!("Part 1: {}", solve(players, marbles));
    println!("Part 2: {}", solve(players, 100*marbles));
}
