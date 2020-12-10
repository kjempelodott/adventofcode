#[macro_use]
extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;

const N: usize = 25;

fn main() {
    let m: Vec<u64> = numbers!(read_from_stdin() => u64);
    let mut i = 0;
    while m[i..i+N].iter()
        .find(|&k| m[i..i+N].iter().find(|&s| m[i+N] == s+k).is_some())
        .is_some() {
            i += 1;
        }
    let invalid = m[i+N];
    println!("Part 1: {}", invalid);

    for i in 0..m.len() {
        let mut j = i+1;
        let mut sum = m[i];
        while sum < invalid {
            sum += m[j];
            j += 1;
        }
        if sum == invalid {
            let (min, max) = (&m[i..j].iter().min(), &m[i..j].iter().max());
            println!("Part 2: {}", min.unwrap() + max.unwrap());
            break;
        }
    }
}
