#![feature(int_roundings)]

extern crate aoc2021;
use aoc2021::read_from_stdin;

type SnailNum = Vec<(usize, usize)>;

fn parse<'a>(line: &'a str) -> SnailNum {
    line.chars()
        .scan(0, |level, c| Some(match c {
            '[' => { *level += 1; None },
            ',' => None,
            ']' => { *level -= 1; None },
            n => Some((*level, (n as u8) as usize - 48))
        }))
        .filter_map(|w| w)
        .collect()
}

fn add(mut a: SnailNum, b: SnailNum) -> SnailNum {
    a.extend(b);
    (0..a.len()).for_each(|i| { a[i].0 += 1; });
    reduce(&mut a);
    a
}

fn reduce(tree: &mut SnailNum) {
    loop {
        if let Some(i) = tree.iter().position(|(lvl,_)| *lvl == 5) { // First element in exploding pair
            let (_, val) = tree.remove(i+1);
            if i < tree.len() - 1 {
                tree[i+1].1 += val;
            }                
            if i > 0 {
                tree[i-1].1 += tree[i].1;
            }
            tree[i] = (4, 0);
            continue;
        }
        if let Some(i) = tree.iter().position(|(_,val)| *val > 9) { // Split
            let (lvl, val) = tree[i];
            let (div, rem) = (val / 2, val % 2);
            tree.insert(i+1, (lvl + 1, div + rem));
            tree[i] = (lvl + 1, div);
            continue;
        }
        break;
    }
}

fn magnitude(mut tree: SnailNum) -> usize {
    (1..=4).rev().for_each(|lvl| {
        while let Some(i) = tree.iter().position(|(l,_)| *l == lvl) {
            let (_, val) = tree.remove(i+1);
            tree[i].1 = 3 * tree[i].1 + 2 * val;
            tree[i].0 -= 1;
        }           
    });
    tree[0].1
}

fn main() {
    let numbers = read_from_stdin().lines()
        .map(|line| parse(line))
        .collect::<Vec<SnailNum>>();

    let result = numbers.iter()
        .cloned()
        .reduce(|t1,t2| add(t1, t2))
        .unwrap();
    println!("Part 1: {}", magnitude(result));

    let mut max = 0;
    for i in 0..numbers.len() {
        for j in i+1..numbers.len() {
            let (a, b) = (numbers[i].clone(), numbers[j].clone());
            let c = magnitude(add(a,b));
            let (a, b) = (numbers[j].clone(), numbers[i].clone());
            let d = magnitude(add(a,b));
            max = std::cmp::max(max, std::cmp::max(c,d));
        }
    }
    println!("Part 2: {}", max);
}
