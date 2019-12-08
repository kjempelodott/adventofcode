extern crate adventofcode2019;
use adventofcode2019::read_from_stdin;

use std::collections::HashMap;

fn path(p: &String, map: &HashMap<String,String>) -> Vec<String> {
    let mut q = p.clone();
    let mut path = vec![q.clone()];
    while q != "COM" {
        q = map.get(&q).unwrap().clone();
        path.push(q.clone());
    }
    path
}

fn main() {
    let mut map: HashMap<String,String> = HashMap::new();
    for link in read_from_stdin().lines() {
        let (a, b) = link.split_at(3);
        map.insert(b[1..].to_string(), a.to_string());
    }   

    println!("Part 1: {}", map.values().fold(0, |sum, p| sum + path(p, &map).len()));

    let you = path(&"YOU".to_string(), &map);
    let san = path(&"SAN".to_string(), &map);
    let nc = you.iter().rev().zip(san.iter().rev()).take_while(|(a,b)| a == b).count();
    println!("Part 2: {}", (san.len() - nc - 1) + (you.len() - nc - 1));
}
