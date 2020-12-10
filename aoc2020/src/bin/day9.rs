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
