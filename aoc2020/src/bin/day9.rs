#[macro_use]
extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;

const N: usize = 25;

fn main() {
    let m: Vec<u64> = numbers!(read_from_stdin() => u64);
    let mut invalid = 0;
    for (i,&n) in m[N..].iter().enumerate() {
        let it = &m[i..i+N].iter();
        if let None = it.clone().find(|&k| it.clone().find(|&s| n == s+k).is_some()) {
            invalid = n;
            break;
        }
    }
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
