#[macro_use]
extern crate aoc2021;
use aoc2021::read_from_stdin;

fn main() {
    let input: Vec<_> = numbers!(read_from_stdin() => usize);
    let mut f = vec![0,0,0,0,0,0,0,0,0];
    for &i in input.iter() {
        f[i] += 1;
    }
    (0..81).for_each(|n| { f[(n+6) % 9] += f[(n+8) % 9]; } );
    println!("Part 1: {}", f.iter().sum::<usize>());
    (81..257).for_each(|n| { f[(n+6) % 9] += f[(n+8) % 9]; } );
    println!("Part 2: {}", f.iter().sum::<usize>());
}
