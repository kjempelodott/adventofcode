extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;

use std::collections::{HashSet,HashMap};

fn main() {
    let mut dfs_map: HashMap<String,Vec<(String,usize)>> = HashMap::new();
    let mut bfs_map: HashMap<String,Vec<String>> = HashMap::new();
    for line in read_from_stdin().lines() {
        let w: Vec<&str> = line.split_whitespace().collect();
        let p = w[0..2].concat();
        let mut children = vec![];
        for i in (7..w.len()).step_by(4) {
            let key = w[i-2..i].concat();
            let num = w[i-3].parse::<usize>().unwrap();
            children.push((key.clone(), num));
            bfs_map.entry(key).or_insert(vec![]).push(p.clone());
        }
        dfs_map.insert(p, children);
    }

    let mut incl: HashSet<String> = bfs_map["shinygold"].iter().cloned().collect();
    let mut bfs = bfs_map["shinygold"].clone();
    while let Some(c) = bfs.pop() {
        if let Some(p) = bfs_map.get(&c) {
            bfs.extend(p.clone());
            incl.extend(p.clone());
        }
    }
    println!("Part 1: {}", incl.len());

    let mut dfs = dfs_map["shinygold"].clone();
    let mut total: usize = dfs.iter().map(|(_,v)| v).sum();
    while let Some((p,m)) = dfs.pop() {
        if let Some(child) = dfs_map.get(&p) {
            for (gc,n) in child.iter() {
                dfs.push((gc.clone(), n*m));
                total += n*m;
            }
        }
    }
    println!("Part 2: {}", total);
}
