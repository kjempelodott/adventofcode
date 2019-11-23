// Keywords: jumps, update items

#[macro_use]
extern crate adventofcode2017;


fn solve<F: Fn(i32) -> i32>(update: F) -> usize {
    let mut instr: Vec<i32> = num_col!(include_str!("../../res/day5") => i32);
    let (mut pos, mut steps) = (0, 0);
    while let Some(jump) = instr.get_mut(pos as usize) {
        pos += *jump;
        *jump += update(*jump);
        steps += 1;
    }
    steps
}

fn main() {
    println!("Part 1: {}", solve(|_| 1));
    println!("Part 2: {}", solve(|v| if v > 2 { -1 } else { 1 }));
}
