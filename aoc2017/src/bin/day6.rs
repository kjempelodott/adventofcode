// Keywords: circular iterable, max, item update, seen states

#[macro_use]
extern crate adventofcode2017;
use std::collections::HashMap;

fn solve(mem: &mut Vec<usize>) -> (usize, usize) {
    let n = mem.len();
    let mut seen: HashMap<Vec<usize>, usize> = HashMap::new();

    for cycles in 0.. {
        if let Some(c) = seen.insert(mem.clone(), cycles) {
            return (seen.len(), seen.len() - c as usize);
        }

        let (i, &max) = ja![mem.iter()
                           .enumerate()
                           .max_by_key(|&(i, &v)| (v, -(i as isize)))];
        (0..n).cycle()
            .skip(i+1)
            .take(max)
            .for_each(|j| mem[j] += 1);
        
        *ja![mem.get_mut(i)] -= max;
    }
    unreachable!();
}

fn main() {
    let (p1, p2) = solve(&mut num_row![include_str!("../../res/day6") => usize]);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(solve(&mut vec![0, 2, 7, 0]), (5, 4));
    }
}
