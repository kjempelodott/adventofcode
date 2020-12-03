extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;

const WIDTH: usize = 31; 

fn traverse(mov: (usize, usize), slope: &Vec<Vec<usize>>) -> usize {
    let (mut y, mut x) = (0, 0);
    let mut trees = 0;
    while y < slope.len() - mov.0 {
        y += mov.0;
        x += mov.1;
        if slope[y].contains(&(x % WIDTH)) {
            trees += 1;
        }
    }
    trees
}

fn main() {
    let slope: Vec<Vec<usize>> = read_from_stdin().lines()
        .map(|line| line.match_indices('#').map(|(i,_)| i).collect())
        .collect();
    println!("Part 1: {}", traverse((1,3), &slope));
    let p: usize = [(1,1), (1,3), (1,5), (1,7), (2,1)].iter()
        .map(|&d| traverse(d, &slope))
        .product();
    println!("Part 2: {}", p);
}
