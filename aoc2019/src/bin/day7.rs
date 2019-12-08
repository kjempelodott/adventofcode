#[macro_use]
extern crate adventofcode2019;
use adventofcode2019::{goddamncomputer,read_from_stdin};

extern crate itertools;
use itertools::Itertools;

fn main() {
    let bytecode = numbers!(read_from_stdin() => isize);   
    let prog = goddamncomputer::Program::new(bytecode);
    println!("Part 1: {:?}", (0..5).permutations(5)
             .map(|phases| phases.iter()
                  .fold(0, |out, &p| prog.run_once(vec![p, out])))
             .max()
             .unwrap());

    // let mut amps: Vec<goddamncomputer::Program> = (0..5).map(|_| prog.clone()).collect();
    // for phases in (5..10).permutations(5) {
    //     let loop1 = phases.iter()
    //         .zip(amps.iter_mut())
    //         .fold(0, |out, (&p, amp)| amp.run(vec![p, out]));
    //     println!("{}", loop1);
    // }
}
