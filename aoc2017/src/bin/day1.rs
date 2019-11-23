// Keywords: circular iterable, item comparison

fn solve(seq: &str, skip: usize) -> u32 {
    let it = seq.bytes().cycle();
    it.clone()
        .take(seq.len())
        .zip(it.skip(skip))
        .map(|(x, y)| if x == y { x as u32 - 48 } else { 0 })
        .sum()
}

fn main() {
    let input = include_str!("../../res/day1");
    println!("Part 1: {}", solve(input, 1));
    println!("Part 2: {}", solve(input, input.len()/2));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part1_example1() {
        assert_eq!(solve("1122", 1), 3);
    }
    #[test]
    fn part1_example2() {
        assert_eq!(solve("1111", 1), 4); 
    }
    #[test]
    fn part1_example3() {
        assert_eq!(solve("1234", 1), 0); 
    }
    #[test]
    fn part1_example4() {
        assert_eq!(solve("91212129", 1), 9);
    }
    #[test]
    fn part2_example1() {
        assert_eq!(solve("1212", 2), 6);
    }
    #[test]
    fn part2_example2() {
        assert_eq!(solve("1221", 2), 0);
    }
    #[test]
    fn part2_example3() {
        assert_eq!(solve("123425", 3), 4);
    }
    #[test]
    fn part2_example4() {
        assert_eq!(solve("123123", 3), 12);
    }
    #[test]
    fn part2_example5() {
        assert_eq!(solve("12131415", 4), 4);
    }
}

