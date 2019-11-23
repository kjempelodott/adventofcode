extern crate adventofcode2018;
use adventofcode2018::read_from_stdin;
use adventofcode2018::metric::{Point,Manhattan};
use std::collections::{HashMap,BinaryHeap};

#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord)]
enum Tile {
    Open,
    Goblin,
    Elf
}
use Tile::*;

#[derive(Debug,PartialEq,Eq,PartialOrd,Ord)]
struct Unit {
    p: Point,
    hp: isize,
    ap: isize,
    t: Tile
}

impl Unit {
    fn new(x: usize, y: usize, ap: isize, t: Tile) -> Self {
        Unit { p: Point::new(y,x), ap: ap, hp: 200, t: t }
    }
    fn get_targets(&self, map: &HashMap<Point,Tile>) -> Vec<Point> {
        let target = if self.t == Elf { Goblin } else { Elf };
        self.p.adjacent().iter().cloned()
            .filter(|s| map.get(&s) == Some(&target))
            .collect()
    }
    fn make_turn(&mut self, map: &mut HashMap<Point,Tile>) -> Vec<Point> {
        let targets = self.get_targets(&map);
        if targets.len() > 0 {
            return targets
        }

        let valid_tiles = map.clone().into_iter()
            .filter(|(_,t)| *t == Open)
            .collect::<HashMap<_,_>>();

        let target = if self.t == Elf { Goblin } else { Elf };
        let mut dist_nearest_target = std::isize::MIN;
        let mut best_step = Point::default();
        let mut visited = HashMap::new();

        for step in &self.p.adjacent() {
            if valid_tiles.get(&step).is_none() {
                continue;
            }
            visited.insert(step.clone(), 0);

            let mut scanner = BinaryHeap::new();
            scanner.push((0, step.clone()));
            while let Some((mut d, pos)) = scanner.pop() {
                d -= 1;
                if d <= dist_nearest_target {
                    continue
                }
                for nb in &pos.adjacent() {
                    if map.get(&nb) == Some(&target) {
                        best_step = step.clone();
                        dist_nearest_target = d;
                        continue
                    }
                    if let Some(prev_d) = visited.get(nb) {
                        if *prev_d >= d {
                            continue
                        }
                    }
                    if valid_tiles.get(&nb).is_none() {
                        continue;
                    }
                    visited.insert(nb.clone(), d);
                    scanner.push((d, nb.clone()));
                }
            }
        }
        if dist_nearest_target != std::isize::MIN {
            map.insert(self.p.clone(), Open);
            self.p = best_step;
            map.insert(self.p.clone(), self.t.clone());
        }
        self.get_targets(&map)
    }
}

fn run_battle(units: &mut Vec<Unit>, map: &mut HashMap<Point,Tile>, strict: bool) -> Option<isize> {
    'round: for mut round in 0.. {
        let mut idx = 0;
        while idx < units.len() {
            let mut targets = units.get_mut(idx).unwrap().make_turn(map);
            // Attack unit with least hp
            targets.sort_by_key(|p| units.iter().find(|u| u.p == *p).unwrap().hp);
            if let Some(a) = targets.get(0) {
                let jdx = units.iter_mut().position(|u| u.p == *a).unwrap();
                units.get_mut(jdx).unwrap().hp -= units[idx].ap;
                if units[jdx].hp <= 0 {
                    if strict && units[jdx].t == Elf {
                        return None
                    }
                    map.insert(units.remove(jdx).p, Open);
                    if jdx < idx {
                        idx -= 1
                    }
                }
            }
            // Did battle end?
            let n_elves = units.iter().filter(|u| u.t == Elf && u.hp > 0).count();
            let n_goblins = units.iter().filter(|u| u.t == Goblin && u.hp > 0).count();
            if n_elves == 0 || n_goblins == 0 {
                if idx == units.len() - 1 { round += 1 };
                return Some(round * units.iter().map(|u| u.hp).sum::<isize>())
            }
            idx += 1;
        }
        units.sort_unstable();
    }
    None
}

fn create_game(input: &String, elf_ap: isize) -> (Vec<Unit>, HashMap<Point,Tile>) {
    let mut map = HashMap::new();
    let mut units = vec![];
    for (y,line) in input.lines().enumerate() {
        for (x,c) in line.chars().enumerate() {
            match c {
                '#' => {},
                '.' => { map.insert(Point::new(y,x), Open); },
                'E' => {
                    map.insert(Point::new(y,x), Elf);
                    units.push(Unit::new(x, y, elf_ap, Elf));
                },
                'G' => {
                    map.insert(Point::new(y,x), Goblin);
                    units.push(Unit::new(x, y, 3, Goblin));
                },
                _ => unreachable!()
            }
        }
    }
    (units, map)
}

fn main() {
    let input = read_from_stdin();
    let (mut units, mut map) = create_game(&input, 3);
    println!("Part 1: {}", run_battle(&mut units, &mut map, false).unwrap());
    for ap in 4.. {
        let (mut units, mut map) = create_game(&input, ap);
        if let Some(result) = run_battle(&mut units, &mut map, true) {
            println!("Part 2: {}", result);
            break
        }
    }
}


