#[macro_use]
extern crate adventofcode2019;
use adventofcode2019::{goddamncomputer,read_from_stdin};

fn main() {
    let bytecode = numbers!(read_from_stdin() => isize);

    let prog = goddamncomputer::Program::new(bytecode);
    println!("Part 1: {}", prog.run(vec![1]));
    println!("Part 1: {}", prog.run(vec![5]));
}
