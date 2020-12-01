#[macro_use]
extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;

fn main() {
    let m: Vec<u64> = numbers!(read_from_stdin() => u64);
    let n = m.iter().find(|&n| m.contains(&(2020-n))).unwrap();
    println!("Part 1: {}", n*(2020-n));
    for &p in m.iter() {
        let r = 2020-p;
        if let Some(&s) = m.iter().filter(|&&n| n < r).find(|&n| m.contains(&(r-n))) {
            println!("Part 2: {}", s*(r-s)*p);            
            break
        }
    }
}
