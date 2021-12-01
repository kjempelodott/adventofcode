#[macro_use]
extern crate aoc2021;
use aoc2021::read_from_stdin;

extern crate itertools;
use itertools::Itertools;

fn main() {
    let n: Vec<u64> = numbers!(read_from_stdin() => u64);
    println!("Part 1: {}", n.iter().skip(1)
             .zip(n.iter())
             .filter(|(&c,&d)| c > d).count());
    //                                                      `\___/`
    //                                                      (O v O)
    println!("Part 2: {}", n.iter().skip(1).tuple_windows::<(_,_,_)>()
    //                                     `\___/`
    //                                     (O v O)
             .zip(n.iter().tuple_windows::<(_,_,_)>())
             .filter(|(v,w)| v.0+v.1+v.2 > w.0+w.1+w.2).count());
}
