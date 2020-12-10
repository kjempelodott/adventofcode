#[macro_use]
extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;

const N: usize = 25;

fn main() {
    let m: Vec<u64> = numbers!(read_from_stdin() => u64);
    let mut i = 0;
    while m[i..i+N].iter()
        .find(|&k| m[i..i+N].iter().find(|&s| m[i] == s+k).is_some())
        .is_some() {
            i += 1;
        }
    println!("Part 1: {}", m[i]);
    for i in 0..m.len() {
        let mut j = i+1;
        let mut sum = m[i];
        while sum < m[i] {
            sum += m[j];
            j += 1;
        }
        if sum == m[i] {
            let (min, max) = (&m[i..j].iter().min(), &m[i..j].iter().max());
            println!("Part 2: {}", min.unwrap() + max.unwrap());
            break;
        }
    }
}
