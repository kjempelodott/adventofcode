use std::collections::HashSet;
use aoc2024::read_from_stdin;

fn main() {
    let mut x = HashSet::new();
    let mut m = HashSet::new();
    let mut a = HashSet::new();
    let mut s = HashSet::new();
    for (j, line) in read_from_stdin().lines().enumerate() {
        for (i, c) in line.chars().enumerate() {
            let (j, i) = (j as isize, i as isize);
            match c {
                'X' => {
                    x.insert((j,i));
                }
                'M' => {
                    m.insert((j,i));
                },
                'A' => {
                    a.insert((j,i));
                },
                'S' => {
                    s.insert((j,i));
                },
                _ => {}
            }
        }
    }

    let mut part1 = 0;
    for (y,x) in x.iter() {
        for (dy,dx) in &[(-1,0), (-1,-1), (0,-1), (1,-1), (1,0), (1,1), (0,1), (-1,1)] {
            if m.contains(&(y+dy,x+dx)) && a.contains(&(y+2*dy,x+2*dx)) && s.contains(&(y+3*dy,x+3*dx)) {
                part1 += 1;
            }
        }
    }
    println!("Part 1: {}", part1);

    let mut part2 = 0;
    for (y,x) in a.iter() {
        if ((m.contains(&(y-1,x-1)) && s.contains(&(y+1,x+1))) || (m.contains(&(y+1,x+1)) && s.contains(&(y-1,x-1)))) &&
            ((m.contains(&(y-1,x+1)) && s.contains(&(y+1,x-1))) || (m.contains(&(y+1,x-1)) && s.contains(&(y-1,x+1))))
        {
            part2 += 1;
        }
    }
    println!("Part 2: {}", part2);
}
