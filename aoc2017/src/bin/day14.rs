// Keywords: ascii, xor, binary, adjacent tiles, regions, dfs

extern crate itertools;
use itertools::Itertools;
use std::collections::{BinaryHeap, BTreeSet};

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

fn knot_hash(input: &Vec<usize>) -> String {
    let mut list: Vec<u32> = (0..N as u32).collect();
    let mut cursor = 0;
    let mut skip = 0;
    for _ in 0..64 {
        for &len in input {
            list = next(&list, cursor, len as usize);
            cursor = (cursor + len as usize + skip) % N;
            skip += 1;
        }
    }

    let mut hash = String::new();
    for chunk in &list.into_iter().chunks(16) {
        let bin = format!("{:08b}", chunk.fold(0, |s, x| s^x));
        hash += &bin;
    }
    hash
}

fn make_grid(input: &str) -> Vec<Vec<char>> {
    let mut ones: Vec<Vec<char>> = vec![];
    for i in 0..128 {
        let iv = format!("{}-{}", input, i);
        let mut bytes: Vec<usize> = iv.bytes().map(|x| x as usize).collect();
        bytes.append(&mut vec![17, 31, 73, 47, 23]);
        ones.push(knot_hash(&bytes).chars().collect());
    }
    ones
}

fn dfs(grid: &Vec<Vec<char>>) -> usize {
    let mut regions: Vec<BTreeSet<(usize, usize)>> = vec![];
    let mut seen: BTreeSet<(usize, usize)> = BTreeSet::new();
    for i in 0..128 {
        for j in 0..128 {
            if seen.contains(&(i, j)) || grid[i][j] == '0' {
                continue
            }
            let mut queue: BinaryHeap<(usize, usize)> = BinaryHeap::new();
            let mut region: BTreeSet<(usize, usize)> = BTreeSet::new();
            queue.push((i, j));
            while let Some((i, j)) = queue.pop() {
                if seen.contains(&(i, j)) || grid[i][j] == '0' {
                    continue
                }
                seen.insert((i, j));
                if i != 127 { queue.push((i+1, j)); }
                if j != 127 { queue.push((i, j+1)); }
                if i != 0 { queue.push((i-1, j)); }
                if j != 0 { queue.push((i, j-1)); }
            }
            regions.push(region);
        }
    }
    regions.len()
}

fn main() {
    let input = "amgozmfv";
    let grid = make_grid(input);
    let part1: usize = grid.iter()
        .map(|row| row.iter().filter(|&c| *c == '1').count())
        .sum();
    println!("Part 1: {}", part1);
    println!("Part 2: {}", dfs(&grid));
}
