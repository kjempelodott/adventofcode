extern crate aoc2021;
use aoc2021::read_from_stdin;

const B: usize = 12;

fn count_ones(bin: &Vec<u16>, i: usize) -> usize {
    bin.iter().filter(|&&r| (1 << i) & r != 0).count()
}

fn main() {
    let bin: Vec<u16> = read_from_stdin()
        .lines()
        .filter_map(|l| u16::from_str_radix(&l, 2).ok())
        .collect();
    let n = bin.len();
    let map: Vec<_> = (0..B).map(|i| count_ones(&bin, i)).collect();

    let gamma = map.iter()
        .enumerate()
        .map(|(i,&v)| if v > (n+1)/2 { 1 << i } else { 0 })
        .sum::<usize>();

    let epsilon = ((1 << B) - 1) ^ gamma;
    println!("Part 1: {}", gamma*epsilon);

    let mut ratings = bin.clone();
    let mut j = B-1;
    while ratings.len() > 1 {
        let n = ratings.len();
        let m = count_ones(&ratings, j);
        if m >= (n+1)/2 {
            ratings = ratings.into_iter().filter(|&r| r & (1 << j) != 0).collect();
        }
        else {
            ratings = ratings.into_iter().filter(|&r| r & (1 << j) == 0).collect();
        }
        j -= 1;
    }
    let oxy = ratings[0] as usize;

    ratings = bin.clone();
    j = B-1;
    while ratings.len() > 1 {
        let n = ratings.len();
        let m = count_ones(&ratings, j);
        if m >= (n+1)/2 {
            ratings = ratings.into_iter().filter(|&r| r & (1 << j) == 0).collect();
        }
        else {
            ratings = ratings.into_iter().filter(|&r| r & (1 << j) != 0).collect();            
        }
        j -= 1;
    }
    let co2 = ratings[0] as usize;

    println!("Part 2: {}", oxy*co2);
}
