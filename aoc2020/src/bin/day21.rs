#![feature(str_split_once)]
#![feature(hash_drain_filter)]

extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;

use std::collections::{BTreeMap,HashSet};

fn main() {
    let foods = read_from_stdin().lines()
        .map(|line| {
            let (list, allergens) = line.split_once('(').unwrap();
            (list.trim().split(' ')
             .map(|s| s.to_string())
             .collect::<Vec<String>>(),
             allergens.trim_matches(')').split(' ')
             .skip(1)
             .map(|s| s.trim_matches(',').to_string())
             .collect::<Vec<String>>())
        })
        .collect::<Vec<_>>();

    let ingredients: HashSet<&String> = foods.iter().map(|(i,_)| i.iter()).flatten().collect();
    let mut allergens: BTreeMap<&String,HashSet<&String>> = BTreeMap::new();

    for (ingrs,alrgs) in foods.iter() {
        for alrg in alrgs.iter() {
            if let Some(seen) = allergens.get_mut(&alrg) {
                seen.drain_filter(|i| !ingrs.contains(&i));
            }
            else {
                allergens.insert(&alrg, ingrs.iter().collect());
            }
        }
    }

    println!("Part 1: {}", ingredients.iter()
             .map(|i|
                  if !allergens.values().any(|a| a.contains(i)) {
                      foods.iter().filter(|(x,_)| x.contains(i)).count()
                  }
                  else {
                      0
                  })
             .sum::<usize>());    

    let mut mapped = allergens.values()
        .filter(|a| a.len() == 1)
        .map(|a| a.iter().next().unwrap().to_string())
        .collect::<Vec<_>>();

    while let Some(a) = mapped.pop() {
        for alrgs in allergens.values_mut().filter(|a| a.len() > 1) {
            if alrgs.remove(&a) && alrgs.len() == 1 {
                mapped.push(alrgs.iter().next().unwrap().to_string())
            }
        }
    }
    //let mut bad_ingrs = ;
    println!("Part 2: {}", allergens.values()
             .map(|v| v.iter().next().unwrap().to_string())
             .collect::<Vec<_>>()
             .join(","));
}
