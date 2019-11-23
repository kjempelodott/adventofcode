// Keywords: ulam spiral, manhattan distance, adjacent tiles

#![feature(iterator_step_by)]

use std::collections::HashMap;

fn new_value(x: i32, y: i32, spiral: &mut HashMap<(i32,i32), u32>) -> u32 {
    let value = [(x,y+1),(x,y-1),(x+1,y),(x-1,y),
                 (x+1,y+1),(x-1,y-1),(x+1,y-1),(x-1,y+1)]
        .iter()
        .map(|adj| spiral.get(adj).unwrap_or(&0))
        .sum();
    spiral.insert((x, y), value);
    value
}

fn solve_part1(num: u32) -> u32 {
    (1..).step_by(2)
        .skip_while(|i| i*i < num)
        .map(|i| (num - (i-2)*(i-2)) % (i-1))
        .next().unwrap()
}

fn solve_part2(num: u32) -> u32 {
    let mut spiral: HashMap<(i32, i32), u32> = HashMap::new();
    spiral.insert((0,0), 1);
    let (mut x, mut y) = (0, 0);

    for i in (1..).step_by(2) {
        for _ in 1..i {
            x -= 1;
            let value = new_value(x, y, &mut spiral);
            if value > num { return value }
        }
        for _ in 1..i {
            y -= 1;
            let value = new_value(x, y, &mut spiral);
            if value > num { return value }
        }
        for _ in 1..i+1 {
            x += 1;
            let value = new_value(x, y, &mut spiral);
            if value > num { return value }
        }
        for _ in 1..i+1 {
            y += 1;
            let value = new_value(x, y, &mut spiral);
            if value > num { return value }
        }
    }
    unreachable!()
}


fn main() {
    let input: u32 = 361527;
    println!("{}", solve_part1(input));
    println!("{}", solve_part2(input));
}

#[cfg(tests)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(1, 0));
        assert_eq!(solve_part1(12, 3));
        assert_eq!(solve_part1(23, 2));
        assert_eq!(solve_part1(10230, 31));
    }
}
