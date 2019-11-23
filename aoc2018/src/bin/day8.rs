#[macro_use]
extern crate adventofcode2018;
use adventofcode2018::read_from_stdin;

// Recursive parsing
fn parse(h: &mut Iterator<Item=usize>) -> (usize, usize) {
    let (c, m) = (h.next().unwrap(), h.next().unwrap());
    let children = (0..c).map(|_| parse(h)).collect::<Vec<(usize,usize)>>();
    let metadata: Vec<usize> = h.take(m).collect();
    let mut meta_sum = metadata.iter().sum::<usize>();
    if c > 0 {
        meta_sum += children.iter().map(|r| r.0).sum::<usize>();
        let value = metadata.into_iter()
            .filter(|&i| i <= c)
            .map(|i| children[(i as isize - 1) as usize].1)
            .sum::<usize>();
        return (meta_sum, value)
    }
    (meta_sum, meta_sum)
}

fn main() {
    let h: Vec<usize> = numbers!(read_from_stdin() => usize);
    let result = parse(&mut h.into_iter());
    println!("Part 1: {}", result.0);
    println!("Part 2: {}", result.1);
}
