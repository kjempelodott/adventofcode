extern crate adventofcode2018;
use adventofcode2018::read_from_stdin;
use adventofcode2018::metric::{Point,Euclidean};
use std::collections::BTreeMap;

#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord)]
enum Tile {
    Open,
    Tree,
    Lumberyard
}
use Tile::*;

fn tick(map: &mut BTreeMap<Point,Tile>) -> BTreeMap<Point,Tile> {
    let snapshot = map.clone();
    for (p, ref mut t) in map.iter_mut() {
        let adj: Vec<&Tile> = p.adjacent().iter()
            .filter_map(|p| snapshot.get(p))
            .collect();
        
        let ntree = adj.iter().filter(|&&t| *t == Tree).count();
        let nlumberyard = adj.iter().filter(|&&t| *t == Lumberyard).count();
        match t {
            Open => { if ntree > 2 { **t = Tree } },
            Tree => { if nlumberyard > 2 { **t = Lumberyard } },
                Lumberyard => { if ntree == 0 || nlumberyard == 0 { **t = Open } }
        }
    }
    snapshot
}

fn main() {
    let mut map = BTreeMap::new();
    for (y,line) in read_from_stdin().lines().enumerate() {
        for (x,c) in line.chars().enumerate() {
            match c {
                '#' => { map.insert(Point::new(y,x), Lumberyard); },
                '.' => { map.insert(Point::new(y,x), Open); },
                '|' => { map.insert(Point::new(y,x), Tree); },
                _ => unreachable!()
            }
        }
    }

    let mut snapshots = vec![];
    (0..10).for_each(|_| { snapshots.push(tick(&mut map)); });

    let ntree = map.values().filter(|&t| *t == Tree).count();
    let nlumberyard = map.values().filter(|&t| *t == Lumberyard).count();
    println!("Part 1: {}", ntree * nlumberyard);

    for i in 10.. {
        let snapshot = tick(&mut map);
        if let Some(idx) = snapshots.iter().position(|ss| *ss == snapshot) {
            let period = i - idx;
            let result = &snapshots[idx + (1000000000 - idx) % period];
            let ntree = result.values().filter(|&t| *t == Tree).count();
            let nlumberyard = result.values().filter(|&t| *t == Lumberyard).count();
            println!("Part 2: {}", ntree * nlumberyard);
            break
        }
        snapshots.push(snapshot);
    }
}
