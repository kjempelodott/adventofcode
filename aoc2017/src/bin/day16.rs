// Keywords: instructions, item swap, rotation, seen states

#![feature(slice_rotate)]
use std::collections::HashMap;

fn solve(input: &str, z: u8, rounds: usize) -> String {
    let p0: Vec<char> = (0u8..z).map(|c| (97 + c) as char).collect();
    let mut seen: HashMap<Vec<char>, usize> = HashMap::new();

    let mut n = 0;
    let mut pn = p0.clone();
    while !seen.contains_key(&pn) && n <= rounds {
        seen.insert(pn.clone(), n);
        n += 1; 
        for i in input.trim().split(",") {
            match i.split_at(1) {
                ("s", x) => {
                    let r = x.parse::<u8>().unwrap() % z;
                    pn.rotate((z - r) as usize);
                }
                ("x", x) => {
                    let ab: Vec<usize> = x.splitn(2, "/")
                        .map(|c| c.parse().unwrap())
                        .collect();
                    pn.swap(ab[0], ab[1]);
                },
                ("p", x) => {
                    let ab: Vec<usize> = x.splitn(2, "/")
                        .map(|c| c.as_bytes()[0] as char)
                        .map(|c| pn.iter().take_while(|&&e| e != c).count())
                    .collect();
                    pn.swap(ab[0], ab[1]);
                },
                _ => unreachable!()
            }
        }
    }
    let i = rounds % n;
    seen.iter().find(|&(_, &v)| v == i).unwrap().0.iter().collect()
}

fn main() {
    let input = include_str!("../../res/day16");
    println!("Part 1: {}", solve(input, 16, 1));
    println!("Part 2: {}", solve(input, 16, 1_000_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_part1() {
        assert_eq!(solve("s1,x3/4,pe/b", 5, 1), "baedc")
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve("s1,x3/4,pe/b", 5, 2), "ceadb")
    }
}
