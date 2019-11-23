// Keywords: generators, bit matching

fn solve_part1(a: u64, b: u64, n: usize) -> usize {
    (0..n)
        .scan((a, b), |ab, _| {
            *ab = ((ab.0*16807) % 2147483647, (ab.1*48271) % 2147483647);
            Some(*ab)
        })
        .filter(|&(a, b)| a & 0xffff == b & 0xffff)
        .count()
}

fn solve_part2(a: u64, b: u64, n: usize) -> usize {
    let gen_a = (0..)
        .scan(a, |a, _| { *a = (*a*16807) % 2147483647; Some(*a) })
        .filter(|&a| a % 4 == 0);
    let gen_b = (0..)
        .scan(b, |b, _| { *b = (*b*48271) % 2147483647; Some(*b) })
        .filter(|&b| b % 8 == 0);
    gen_a.zip(gen_b)
        .take(n)
        .filter(|&(a, b)| a & 0xffff == b & 0xffff)
        .count()
}

fn main() {
    println!("Part 1: {}", solve_part1(883, 879, 40_000_000));
    println!("Part 2: {}", solve_part2(883, 879, 5_000_000));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_part1() {
        assert_eq!(solve_part1(65, 8921, 5), 1)
    }
    #[test]
    fn example_part2() {
        assert_eq!(solve_part2(65, 8921, 5_000_000), 309)
    }
}
