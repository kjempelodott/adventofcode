extern crate adventofcode2018;
use adventofcode2018::read_from_stdin;
use adventofcode2018::metric::Point;
use adventofcode2018::metric::Direction::*;
use std::collections::BTreeMap;

fn branch(map: &mut BTreeMap<Point, usize>,
          _cur: &mut Point,
          _doors: &mut usize,
          it: &mut Iterator<Item=char>) {

    let mut cur = _cur.clone();
    let mut doors = *_doors;
    let mut shortest = doors;
    while let Some(c) = it.next() {
        match c {
            'S'|'N'|'W'|'E' => {
                let dir = match c { 'S'=>Down, 'N'=>Up,
                                    'W'=>Left, 'E'=>Right, _=>unreachable!() };
                cur.mov(dir); cur.mov(dir);
                doors += 1;
                let room = cur.clone();
                if *map.get(&room).unwrap_or(&std::usize::MAX) > doors {
                    // Found shorter path to room
                    map.insert(room, doors);
                }
            },
            '(' => {
                branch(map, &mut cur, &mut doors, it);
            },
            '|' => {
                // Reset
                if doors < shortest { shortest = doors; }
                cur = _cur.clone();
                doors = _doors.clone();
            },
            ')' => {
                // Set final location and break
                *_cur = cur;
                *_doors = shortest;
                return
            },
            _ => {}
        }
    }
}

fn main() {
    let input = read_from_stdin();
    let mut it = input.chars();
    let mut map: BTreeMap<Point, usize> = BTreeMap::new();
    let mut cur = Point::default();
    let mut doors = 0;
    branch(&mut map, &mut cur, &mut doors, &mut it);
    println!("Part 1: {}", map.values().max().unwrap());
    println!("Part 2: {}", map.values().filter(|&v| *v >= 1000).count());
}
