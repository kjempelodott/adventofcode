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

    let (mut a, mut b) = (i+N-2, i+N-1);
    let mut sum = m[b];
    loop {
        while sum < invalid {
            sum += m[a];
            a -= 1;
        }
        if sum == invalid {
            a += 1;
            let (min, max) = (&m[a..b].iter().min(), &m[a..b].iter().max());
            println!("Part 2: {}", min.unwrap() + max.unwrap());
            break
        }
        sum -= m[b];
        b -= 1;
    }
}
