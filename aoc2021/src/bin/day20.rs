extern crate aoc2021;
use aoc2021::read_from_stdin;

use std::collections::HashSet;

type P = (isize, isize);
const DXDY: [P; 9] = [(-1,-1),(0,-1),(1,-1),
                      (-1, 0),(0, 0),(1, 0),
                      (-1, 1),(0, 1),(1, 1)];

fn run<'a>(im: HashSet<P>, algo: &'a str, step: u8) -> HashSet<P> {
    let mut enhanced = HashSet::new();
    let (x0,x1,y0,y1) = (
        im.iter().min_by_key(|(i,_)| i).unwrap().0,
        im.iter().max_by_key(|(i,_)| i).unwrap().0,
        im.iter().min_by_key(|(_,j)| j).unwrap().1,
        im.iter().max_by_key(|(_,j)| j).unwrap().1
    );
    for i in x0-1..=x1+1 {
        for j in y0-1..=y1+1 {
            let val = DXDY.iter().enumerate()
                .map(|(k, (dx,dy))| {
                    let x = dx + i;
                    let y = dy + j;
                    return if x < x0 || x > x1 || y < y0 || y > y1 {
                        if step % 2 == 1 { 1 } else { 0 }
                    }
                    else {
                        if im.contains(&(x, y)) { 1 } else { 0 }
                    }
                    << (8-k)
                })
                .sum();
            if algo.chars().nth(val).unwrap() == '#' {
                enhanced.insert((i, j));
            }
        }
    }
    enhanced
}

fn main() {
    let input = read_from_stdin();
    let (algo, image) = input.split_once("\n\n").unwrap();
    let im = image.lines()
        .enumerate()
        .flat_map(|(j,l)| l.chars()
                  .enumerate()
                  .filter(|(_,c)| *c == '#')
                  .map(move |(i,_)| (i as isize, j as isize))
        )
        .collect::<HashSet<P>>();
    let two_steps = (0..=1).fold(im, |enh, s| run(enh, algo, s));
    println!("Part 1: {}", two_steps.len());
    println!("Part 2: {}", (2..50).fold(two_steps, |enh, s| run(enh, algo, s)).len());
}
