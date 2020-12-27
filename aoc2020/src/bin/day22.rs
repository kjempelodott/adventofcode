#![feature(str_split_once)]
#[macro_use]
extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;

use std::hash::{Hash,Hasher};
use std::collections::hash_map::DefaultHasher;
use std::collections::VecDeque;

type Game = fn(&mut VecDeque<usize>, &mut VecDeque<usize>) -> Winner;

enum Winner {
    Player1,
    Player2
}

fn hash(a: &VecDeque<usize>, b: &VecDeque<usize>) -> u64 {
    let mut hasher = DefaultHasher::new();
    a.hash(&mut hasher);
    b.hash(&mut hasher);
    hasher.finish()
}

fn combat(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>) -> Winner {
    while !p1.is_empty() && !p2.is_empty() {
        let (c1, c2) = (p1.pop_front().unwrap(), p2.pop_front().unwrap());
        if c1 > c2 {
            p1.push_back(c1);
            p1.push_back(c2);
        }
        else {
            p2.push_back(c2);
            p2.push_back(c1);
        }
    }
    if p2.is_empty() {
        return Winner::Player1;
    }
    Winner::Player2
}

fn recursive_combat(p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>) -> Winner {
    let mut seen = Vec::with_capacity(p1.len() + p2.len());
    while !p1.is_empty() && !p2.is_empty() {
        let state = hash(&p1, &p2);
        if seen.contains(&state) {
            return Winner::Player1;
        }
        seen.push(state);
        let (c1, c2) = (p1.pop_front().unwrap(), p2.pop_front().unwrap());

        let mut result = Winner::Player1;
        if p1.len() >= c1 && p2.len() >= c2 {
            let mut s1 = p1.iter().take(c1).cloned().collect();
            let mut s2 = p2.iter().take(c2).cloned().collect();
            result = recursive_combat(&mut s1, &mut s2);
        }
        else if c2 > c1 {
            result = Winner::Player2;
        }
        match result {
            Winner::Player1 => {
                p1.push_back(c1);
                p1.push_back(c2);
            },
            Winner::Player2 => {
                p2.push_back(c2);
                p2.push_back(c1);
            }
        }
    }
    if p2.is_empty() {
        return Winner::Player1;
    }
    Winner::Player2
}

fn play(mut p1: VecDeque<usize>, mut p2: VecDeque<usize>, f: Game) -> usize {
    match f(&mut p1, &mut p2) {
        Winner::Player1 => p1.iter().rev().enumerate().map(|(x,y)| (x+1)*y).sum::<usize>(),
        Winner::Player2 => p2.iter().rev().enumerate().map(|(x,y)| (x+1)*y).sum::<usize>(),
    }
}

fn main() {
    let input = read_from_stdin();
    let (cards1, cards2) = input.split_once("\n\n").unwrap();
    let mut p1: VecDeque<usize> = numbers!(cards1 => usize);
    let mut p2: VecDeque<usize> = numbers!(cards2 => usize);
    p1.pop_front(); p2.pop_front();
    println!("Part 1: {}", play(p1.clone(), p2.clone(), combat));
    println!("Part 2: {}", play(p1, p2, recursive_combat));
}
