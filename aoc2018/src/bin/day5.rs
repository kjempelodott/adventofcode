extern crate adventofcode2018;
use adventofcode2018::read_from_stdin;

fn react(poly: &Vec<i8>) -> usize {
    poly.iter().fold(vec![0], |mut p, x| {
        if (x - p[p.len() - 1]).abs() == 32 { p.pop(); }
        else { p.push(*x) }
        p
    }).len() - 1
}

fn main() {
    let bytes: Vec<i8> = read_from_stdin().trim().bytes()
        .map(|b| b as i8)
        .collect();
    println!("Part 1: {}", react(&bytes));

    println!("Part 2: {}", (97..123)
             .map(|i| react(&bytes.clone().into_iter()
                            .filter(|&b| (b - i) % 32 != 0)
                            .collect()))
             .min()
             .unwrap());
}
