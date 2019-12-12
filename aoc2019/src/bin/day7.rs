#[macro_use]
extern crate adventofcode2019;
use adventofcode2019::read_from_stdin;
use adventofcode2019::goddamncomputer::{Program,State};

extern crate itertools;
use itertools::Itertools;

fn main() {
    let intcode = numbers!(read_from_stdin() => isize);
    let prog = Program::new(intcode);
    println!("Part 1: {}", (0..5).permutations(5)
             .map(|phases| phases.iter()
                  .fold(0, |out, &p| prog.run_once(vec![p, out])[0]))
             .max()
             .unwrap());

    println!("Part 2: {}", (5..10).permutations(5)
             .map(|phases| {
                 let mut amps: Vec<Program> = (0..5).map(|_| prog.clone()).collect();
                 let mut signal = phases.iter()
                     .zip(amps.iter_mut())
                     .fold(0, |out, (&p, amp)| amp.run(vec![p, out])[0]);
                 while amps[4].get_state() != State::Halted {
                     signal = amps.iter_mut().fold(signal, |out, amp| amp.run(vec![out])[0]);
                 }
                 signal
             })
             .max()
             .unwrap());
}
