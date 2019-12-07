#![feature(is_sorted)]
extern crate counter;
use counter::Counter;

fn main() {
    println!("Part 1: {}", (273025..767253)
             .map(|i| i.to_string().into_bytes())
             .filter(|i| {
                 let mut c = i.clone();
                 c.dedup();
                 i.is_sorted() && c.len() < 6
             })
             .count());

    println!("Part 2: {}", (273025..767253)
             .map(|i| i.to_string().into_bytes())
             .filter(|i| {
                 let c = i.iter().collect::<Counter<_>>();
                 i.is_sorted() && c.values().any(|&n| n == 2)
             })
             .count());
}
