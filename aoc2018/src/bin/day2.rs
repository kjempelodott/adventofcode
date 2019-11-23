extern crate adventofcode2018;
extern crate counter;
extern crate itertools;
use adventofcode2018::{into_lines,read_from_stdin};
use counter::Counter;
use itertools::Itertools;

fn main() {
    let boxes: Vec<String> = into_lines(read_from_stdin());
    let (mut n2, mut n3) = (0, 0);
    for b in boxes.iter() {
        let cc = b.chars().collect::<Counter<char>>();
        n2 += if cc.values().any(|&v| v == 2) { 1 } else { 0 };
        n3 += if cc.values().any(|&v| v == 3) { 1 } else { 0 };
    }
    println!("Part 1: {}", n2 * n3);

    for (box1, box2) in  boxes.iter().tuple_combinations() {
        let union = box1.chars()
            .zip(box2.chars())
            .filter_map(|(c1, c2)| if c1 == c2 { Some(c1) } else { None });
        if union.clone().count() == 25 {
            println!("Part 2: {}", union.collect::<String>());
            break
        }
    }
}
