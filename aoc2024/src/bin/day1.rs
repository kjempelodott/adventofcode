use counter::Counter;
use aoc2024::{numbers,read_from_stdin};

fn main() {
    let (mut l1, mut l2): (Vec<_>, Vec<_>) = read_from_stdin()
        .lines()
        .map(|line| numbers!(line => isize))
        .map(|num: Vec<isize>| (num[0], num[1]))
        .unzip();
                                     
    l1.sort();
    l2.sort();
    let part1 = l1.iter()
        .zip(l2.iter())
        .map(|(n1,n2)| isize::abs(n1 - n2))
        .sum::<isize>();
    println!("Part 1: {}", part1);

    let c1 = l1.iter().collect::<Counter<_>>();
    let c2 = l2.iter().collect::<Counter<_>>();
    let part2 = c1.iter().map(|(&v,&m)| *v as usize * c2[v] * m).sum::<usize>();
    println!("Part 2: {}", part2);
}
