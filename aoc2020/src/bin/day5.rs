extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;

fn main() {
    // row: 7 first bits
    // col: 3 last bits
    // rowid * 8 == row << 3
    let mut sids: Vec<u16> = read_from_stdin().lines()
        .map(|spec| spec.bytes()
             .map(|b| 1-((b & 4)>>2) as u16) // F|L & 4 = 4, B|R & 4 = 0
             .fold(0, |sid, b| (sid<<1) + b)) // rowid is automatically bitshifted by << 3
        .collect();
    sids.sort();
    println!("Part 1: {}", sids.last().unwrap());
    println!("Part 2: {}", sids.windows(2)
             .filter(|w| w[1] == w[0]+2)
             .next().unwrap()[0] + 1
    );
}
