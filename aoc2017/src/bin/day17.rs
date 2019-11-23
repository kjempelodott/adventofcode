// Keywords: circular iterable, item update, insertion

use std::collections::VecDeque;

fn solve(step: usize) -> (usize, usize) {
    let mut buf: VecDeque<usize> = VecDeque::with_capacity(2017);
    buf.push_back(0);
    let mut pos = 0;
    for i in 1..2018 {
        pos = 1 + (pos + step) % i;
        buf.insert(pos, i);
    }
    let part1 = buf[buf.iter().position(|&v| v == 2017).unwrap() + 1];
    for i in 2018..50_000_000 {
        pos = 1 + (pos + step) % i;
        if pos == 1 {
            buf.insert(1, i)
        }
    }
    (part1, buf[1])
}

fn main() {
    println!("{:?}", solve(355));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        assert_eq!(solve(3).0, 638);
    }
}
