use aoc2024::{numbers,read_from_stdin};
use cached::proc_macro::cached;

#[cached(size=100000)]
fn expand(stone: usize, n: usize, max: usize) -> usize {
    if n == max {
        return 1;
    }
    if stone == 0 {
        return expand(1, n+1, max);
    }
    for i in 0usize.. {
        if stone < usize::pow(10, i as u32) {
            match i % 2 {
                0 => {
                    let s = usize::div_ceil(i, 2);
                    let pow = usize::pow(10, s as u32);
                    let r = stone % pow;
                    return expand(r, n+1, max) + expand((stone - r) / pow, n+1, max);
                }
                _ => {}
            }
            break;
        }
    }
    return expand(stone * 2024, n+1, max);
}

fn main() {
    let stones: Vec<_> = numbers!(read_from_stdin() => usize);
    println!("Part 1: {}", stones.iter().map(|s| expand(*s, 0, 25)).sum::<usize>());
    println!("Part 2: {}", stones.iter().map(|s| expand(*s, 0, 75)).sum::<usize>());
}
