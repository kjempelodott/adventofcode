#[macro_use]
extern crate adventofcode2019;
use adventofcode2019::{goddamncomputer,read_from_stdin};

fn main() {
    let intcode = numbers!(read_from_stdin() => isize);

    let prog = goddamncomputer::Program::new(intcode);
    println!("Part 1: {}", prog.run_once(vec![1]));
    println!("Part 1: {}", prog.run_once(vec![5]));
}
