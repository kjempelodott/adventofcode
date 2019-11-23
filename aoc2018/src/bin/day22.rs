#[macro_use]
extern crate adventofcode2018;
use adventofcode2018::read_from_stdin;
use adventofcode2018::metric::{Point,Manhattan};
use adventofcode2018::metric::Direction::*;
use std::collections::{BinaryHeap,HashMap};

#[derive(Clone,Copy,Debug,PartialEq)]
enum TileType {
    Rocky,
    Wet,
    Narrow
}
use TileType::*;

#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
enum Tool {
    ClimbingGear,
    Torch,
    Neither
}
use Tool::*;

#[derive(Clone,Debug)]
struct Tile {
    geo_idx: isize,
    erosion: isize,
    tt: TileType
}

#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord,Hash)]
struct Hero {
    p: Point,
    tool: Tool
}

type Map = HashMap<Point,Tile>;

impl Tile {
    fn erosion_as_type(erosion: isize) -> TileType {
        match erosion % 3 {
            0 => Rocky,
            1 => Wet,
            2 => Narrow,
            _ => unreachable!()
        }
    }
    fn get_type(p: &Point, d: isize, map: &mut Map) -> TileType {
        if let Some(tile) = map.get(&p) {
            return tile.tt;
        }
        Tile::erosion_as_type(Tile::get_erosion(&p, d, map))
    }
    fn get_erosion(p: &Point, d: isize, map: &mut Map) -> isize {
        (Tile::get_index(&p, d, map) + d) % 20183
    }
    fn get_index(p: &Point, d: isize, map: &mut Map) -> isize {
        if let Some(tile) = map.get(&p) {
            return tile.geo_idx;
        }
        let geo_idx =
            if p.x == 0 {
                p.y * 48271
            } else if p.y == 0 {
                p.x * 16807
            } else { Tile::get_erosion(&p.next_to(Up), d, map) *
                     Tile::get_erosion(&p.next_to(Left), d, map)
            };
        let erosion = (d + geo_idx) % 20183;
        let tt = Tile::erosion_as_type(erosion);
        map.insert(*p, Tile { geo_idx: geo_idx, erosion: erosion, tt: tt });
        geo_idx
    }
}

fn main() {
    let input = numbers![read_from_stdin() => isize];
    let depth = input[0];
    let target = Point::new(input[2],input[1]);

    let mut map = HashMap::new();
    // Recursivley populate rectangle
    Tile::get_index(&target, depth, &mut map);
    // Target is always rocky and with a geological index of 0
    map.insert(target, Tile { geo_idx: 0, erosion: depth % 20183, tt: Rocky });
    println!("Part 1: {}", map.values().map(|t| t.tt as usize).sum::<usize>());

    let mut visited = HashMap::new();
    let mut scanner = BinaryHeap::new();
    let mut current_shortest = std::isize::MIN;
    scanner.push((0, Hero { p: Point::default(), tool: Torch }));
    while let Some((d, cur)) = scanner.pop() {
        if current_shortest >= d {
            continue
        }
        if cur.p == target && cur.tool == Torch {
            current_shortest = d;
            continue
        }
        if let Some(prev_d) = visited.get(&cur) {
            if *prev_d >= d {
                continue
            }
        }
        for adj in &cur.p.adjacent() {
            if adj.x < 0 || adj.y < 0 {
                continue
            }
            match Tile::get_type(&adj, depth, &mut map) {
                Rocky => {
                    if cur.tool == Neither {
                        continue
                    }
                    let torch = Hero { p: adj.clone(), tool: Torch };
                    let climb = Hero { p: adj.clone(), tool: ClimbingGear };
                    scanner.push((d - if cur.tool == Torch { 1 } else { 8 }, torch));
                    scanner.push((d - if cur.tool == ClimbingGear { 1 } else { 8 }, climb));
                },
                Wet => {
                    if cur.tool == Torch {
                        continue
                    }
                    let climb = Hero { p: adj.clone(), tool: ClimbingGear };
                    let neither = Hero { p: adj.clone(), tool: Neither };
                    scanner.push((d - if cur.tool == ClimbingGear { 1 } else { 8 }, climb));
                    scanner.push((d - if cur.tool == Neither { 1 } else { 8 }, neither));
                },
                Narrow => {
                    if cur.tool == ClimbingGear {
                        continue
                    }
                    let torch = Hero { p: adj.clone(), tool: Torch };
                    let neither = Hero { p: adj.clone(), tool: Neither };
                    scanner.push((d - if cur.tool == Torch { 1 } else { 8 }, torch));
                    scanner.push((d - if cur.tool == Neither { 1 } else { 8 }, neither));
                }
            }
        }
        visited.insert(cur, d);
    }
    println!("Part 2: {}", -current_shortest);
}
