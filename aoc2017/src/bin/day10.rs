// Keywords: circular iterable, rotation, ascii, xor, hex

extern crate itertools;
use itertools::Itertools;

const N: usize = 256;

fn next(list: &Vec<u32>, cursor: usize, len: usize) -> Vec<u32>{
    let it = list.iter().cycle().skip(cursor);
    let rev: Vec<&u32> = it.clone().take(len).collect();
    rev.into_iter().rev()
        .chain(it.skip(len).take(N - len))
        .cycle().skip(N - cursor)
        .take(N)
        .map(|x| *x)
        .collect()
}

fn solve_part1(input: Vec<usize>) -> u32 {
    let mut list: Vec<u32> = (0..N as u32).collect();
    let mut cursor = 0;    
    for (i, &len) in input.iter().enumerate() {
        list = next(&list, cursor, len);
        cursor = (cursor + len + i) % N;
    }
    list[0] * list[1]
}

fn solve_part2(input: Vec<usize>) -> String {
    let mut list: Vec<u32> = (0..N as u32).collect();
    let mut cursor = 0;
    let mut skip = 0;
    for _ in 0..64 {
        for &len in input.iter() {
            list = next(&list, cursor, len);
            cursor = (cursor + len + skip) % N;
            skip += 1;
        }
    }

    let mut hash = String::new();
    for chunk in &list.into_iter().chunks(16) {
        let hex = format!("{:02x}", chunk.fold(0, |s, x| s^x));
        hash += &hex;
    }
    hash
}

fn main() {
    let input = include_str!("../../res/day10").trim();
    let part1 = input.split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect();
    let mut part2: Vec<usize> = input.bytes().map(|x| x as usize).collect();
    part2.append(&mut vec![17, 31, 73, 47, 23]);

    println!("Part 1: {}", solve_part1(part1));
    println!("Part 2: {}", solve_part2(part2));
}
