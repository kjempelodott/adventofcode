use std::collections::{HashSet,BinaryHeap};
use aoc2024::read_from_stdin;

type C = (isize, isize);

fn main() {
    let input = read_from_stdin();
    let max_x = input.lines().next().unwrap().len() as isize;
    let max_y = input.lines().count() as isize;

    let map = input.lines()
        .map(|l| l.bytes().map(|c| c - 48).collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    let trailheads = map.iter()
        .enumerate()
        .map(|(j,row)| row.iter()
             .enumerate()
             .filter_map(move |(i,c)| if *c == 0 { Some((j as isize, i as isize)) } else { None })
        )
        .flatten()
        .collect::<Vec<C>>();

    let mut score = 0;
    let mut rating = 0;
    for p in trailheads.iter().cloned() {
        let mut visited = HashSet::new();
        let mut trails = BinaryHeap::new();
        trails.push((0,p));

        while let Some((n,pos)) = trails.pop() {
            let dd = [(-1,0), (0,1), (1,0), (0,-1)];
            for d in dd.iter() {
                let q = (pos.0 + d.0, pos.1 + d.1);
                if q.0 < max_y &&  q.0 >= 0 && q.1 < max_x && q.1 >= 0 {
                    let m = map[q.0 as usize][q.1 as usize];
                    if m == n+1 {
                        if m == 9 {
                            rating += 1;
                            if visited.insert(q) == true {
                                score += 1;
                            }
                        }
                        else {
                            trails.push((n+1,q))
                        }
                    }
                }
            }
        }
    }
    println!("Part 1: {}", score);
    println!("Part 2: {}", rating);
}
