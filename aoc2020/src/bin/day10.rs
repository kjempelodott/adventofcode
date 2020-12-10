#[macro_use]
extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;

extern crate itertools;
use itertools::Itertools;

extern crate counter;
use counter::Counter;

fn main() {
    let mut a = vec![0];
    a.extend(numbers!(read_from_stdin() => u64));
    a.sort();
    a.push(a[a.len()-1] + 3);
    let d = &a[1..].iter().zip(&a[..a.len()]).map(|(b,c)| b - c).collect::<Vec<_>>();
    let c = d.iter().collect::<Counter<_>>();
    println!("Part 1: {}", c[&1] * c[&3]);

    let mut mul = 1;
    for (b, c) in &d.into_iter().group_by(|&&v| v == 1) {
        if b {
            mul *= (1..c.count()).sum::<usize>() + 1;
        }
    }
    println!("Part 2: {}", mul);
}
