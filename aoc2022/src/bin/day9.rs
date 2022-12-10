extern crate aoc2022;
use aoc2022::{read_from_stdin,into_lines};
use std::collections::HashSet;

type Point = (isize, isize);

fn mov(dir: (isize, isize), n: usize, rope: &mut Vec<Point>, visited: &mut HashSet<Point>) {
    let r = rope.len();
    for _ in 0..n {
        rope[0] = (rope[0].0 + dir.0, rope[0].1 + dir.1);
        for i in 1..r {
            let v_diff = rope[i-1].0 - rope[i].0;
            let h_diff = rope[i-1].1 - rope[i].1;
            if v_diff.abs() + h_diff.abs() > 2 {
                rope[i].0 += v_diff/v_diff.abs();
                rope[i].1 += h_diff/h_diff.abs();
            }
            else if v_diff.abs() == 2 {
                rope[i].0 += v_diff/v_diff.abs();
            }
            else if h_diff.abs() == 2 {
                rope[i].1 += h_diff/h_diff.abs();
            }
        }
        visited.insert(rope[r-1]);
    }
}

fn run(rope_len: usize, input: &Vec<String>) -> usize {
    let mut rope = vec![(0,0); rope_len];
    let mut visited: HashSet<Point> = HashSet::new();
    for line in input.iter() {
        let (dir, mag) = line.split_once(' ').unwrap();
        let n = mag.parse::<usize>().unwrap();
        match dir {
            "L" => mov((0,-1), n, &mut rope, &mut visited),
            "R" => mov((0,1), n, &mut rope, &mut visited),
            "U" => mov((-1,0), n, &mut rope, &mut visited),
            "D" => mov((1,0), n, &mut rope, &mut visited),
            _ => unreachable!()
        };
    }
    visited.len()
}

fn main() {
    let input = into_lines(read_from_stdin());
    println!("Part 1: {}", run(2, &input));
    println!("Part 2: {}", run(10, &input));
}
