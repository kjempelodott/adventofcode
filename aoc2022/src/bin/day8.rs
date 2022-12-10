extern crate aoc2022;
use aoc2022::read_from_stdin;
use std::collections::HashSet;

type Point = (usize, usize);

fn main() {
    let trees = read_from_stdin().lines()
        .map(|line| line.bytes().map(|b| b - 48).collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();
    let (sy, sx) = (trees.len(), trees[0].len());
    // Edges are all visible
    let mut visible_forest: HashSet<Point> = (0..sx).map(|x| [(0,x),(sy-1,x)])
        .chain((0..sy).map(|y| [(y,0),(y,sx-1)]))
        .flatten()
        .collect();

    let mut max_score = 0;
    for y in 1..sy-1 {
        for x in 1..sx-1 {
            let tree = trees[y][x];
            if (0..x).all(|k| trees[y][k] < tree) || (x+1..sx).all(|k| trees[y][k] < tree) {
                visible_forest.insert((y,x));
            }
            if (0..y).all(|j| trees[j][x] < tree) || (y+1..sy).all(|j| trees[j][x] < tree) {
                visible_forest.insert((y,x));
            }
            let mut score = 1;
            // Edge or first taller is always visible
            score *= 1 + (x+1..sx-1).take_while(|&k| trees[y][k] < tree).count();
            score *= 1 + (1..x).rev().take_while(|&k| trees[y][k] < tree).count();
            score *= 1 + (y+1..sy-1).take_while(|&j| trees[j][x] < tree).count();
            score *= 1 + (1..y).rev().take_while(|&j| trees[j][x] < tree).count();
            max_score = std::cmp::max(score, max_score);
        }
    }
    println!("Part 1: {}", visible_forest.len());
    println!("Part 2: {}", max_score);
}
