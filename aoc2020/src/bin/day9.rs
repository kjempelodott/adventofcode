#[macro_use]
extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;

const N: usize = 25;

fn main() {
    let m: Vec<u64> = numbers!(read_from_stdin() => u64);
    let i = (0..).find(|&i| {
        (i..i+N).find(|&j| {
            (j+1..i+N).find(|&k| m[i+N] == m[j] + m[k]).is_some()
        }).is_none()
    }).unwrap();
    let invalid = m[i+N];
    println!("Part 1: {}", invalid);

    let (a, b) = (0..i+N-1).rev().find_map(|b| {
        let (mut a, mut sum) = (b-1, m[b]);
        while sum < invalid {
            sum += m[a];
            a -= 1;
        }
        if sum == invalid {
            return Some((a+1, b))
        }
        None
    }).unwrap();
    let (min, max) = (&m[a..b].iter().min(), &m[a..b].iter().max());
    println!("Part 2: {}", min.unwrap() + max.unwrap());
}
