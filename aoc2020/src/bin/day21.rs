#![feature(str_split_once)]
#![feature(hash_drain_filter)]

extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;

use std::collections::{BTreeMap,HashMap,HashSet};

fn main() {
    let mut allergen_map: BTreeMap<String,HashSet<String>> = BTreeMap::new();
    let mut i_count: HashMap<String,usize> = HashMap::new();

    for l in read_from_stdin().lines() {
        let (isub, asub) = l.split_once(" (contains ").unwrap();
        let ingredients = isub.split(' ')
            .map(|s| s.to_string())
            .collect::<HashSet<_>>();
        let allergens = asub.trim_matches(')').split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        for i in ingredients.iter() {
            *i_count.entry(i.to_string()).or_default() += 1;
        }
        for a in allergens.iter() {
            allergen_map.entry(a.to_string())
                .and_modify(|x| { x.drain_filter(|i| !ingredients.contains(i)); } )
                .or_insert(ingredients.clone());
        }
    }
    println!("Part 1: {}", i_count.iter()
             .filter(|(i,_)| !allergen_map.values().any(|a| a.contains(*i)))
             .map(|(_,v)| v)
             .sum::<usize>());

    let mut mapped = allergen_map.values()
        .filter(|a| a.len() == 1)
        .map(|a| a.iter().next().unwrap().to_string())
        .collect::<Vec<_>>();
    for a in allergen_map.values_mut().filter(|a| a.len() > 1) {
        a.drain_filter(|x| mapped.contains(&x));
        if a.len() == 1 {
            mapped.push(a.iter().next().unwrap().to_string());
        }
    }
    println!("Part 2: {}", allergen_map.values()
             .map(|v| v.iter().next().unwrap().to_string())
             .collect::<Vec<_>>()
             .join(","));
}
