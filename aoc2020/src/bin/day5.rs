extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;

fn main() {
    // row: 7 first bits
    // col: 3 last bits
    // rowid * 8 == row << 3
    let mut sids: Vec<u16> = read_from_stdin().lines()
        .map(|spec| spec.chars()
             .map(|c| match c { 'F'|'L' => 0, 'B'|'R' => 1, _ => unreachable!() })
             .fold(0, |sid, b| (sid<<1) + b)) // rowid is automatically bitshifted by << 3
        .collect();
    sids.sort();
    println!("Part 1: {}", sids.last().unwrap());
    println!("Part 2: {}", sids.windows(2)
             .filter(|w| w[1] == w[0]+2)
             .map(|w| w[0])
             .next().unwrap() + 1
    );
}
