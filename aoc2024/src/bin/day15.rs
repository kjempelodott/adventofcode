use itertools::Itertools;
use std::collections::HashMap;
use std::fmt;
use aoc2024::read_from_stdin;

type C = (isize, isize);
type EntityIndex = usize;

#[derive(Clone,Copy,Debug,Default,Eq,PartialEq)]
struct Entity {
    movable: bool,
    y: isize,
    x: isize
}

impl Entity {
    fn movable(pos: C) -> Self {
        Entity {
            movable: true,
            y: pos.0,
            x: pos.1
        }
    }

    fn wall(pos: C) -> Self {
        Entity {
            movable: false,
            y: pos.0,
            x: pos.1
        }
    }
}

struct Map {
    entities: Vec<Entity>,
    map: HashMap<C,EntityIndex>,
    width: isize,
    max_pos: C
}

impl Map {
    fn new(input: &str, w: isize) -> Self {
        let mut map = HashMap::new();
        let mut entities = vec![];
        let mut robot = Entity::movable((0,0));
        let max_x = w * input.lines().next().unwrap().len() as isize;
        let max_y = input.lines().count() as isize;
        
        for (j,line) in input.lines().enumerate() {
            for (i,c) in line.chars().enumerate() {
                let j = j as isize;
                let i = i as isize * w;
                match c {
                    '#' => {
                        (0..w).for_each(|s| {
                            map.insert((j,i+s), entities.len());
                        });
                        entities.push(Entity::wall((j,i)));
                    }
                    'O' => {
                        (0..w).for_each(|s| {
                            map.insert((j,i+s), entities.len());
                        });
                        entities.push(Entity::movable((j,i)));
                    },
                    '@' => {
                        robot.y = j;
                        robot.x = i;
                    },
                    _ => {}
                }
            }
        }
        entities.push(robot);
        Map {
            entities: entities,
            map: map,
            width: w,
            max_pos: (max_y, max_x)
        }
    }

    fn move_robot(&mut self, dir: C) {
        if let Some(to_move) = self._move(self.entities.len() - 1, dir, 1) {
            // Move boxes
            let (dy, dx) = dir;
            for idx in to_move.clone().into_iter().skip(1).unique() {
                let ref e = &self.entities[idx];
                for s in 0..self.width {
                    self.map.remove(&(e.y,e.x+s));
                }
            }
            for idx in to_move.clone().into_iter().skip(1).unique() {
                let ref mut e = &mut self.entities[idx];
                e.y += dy;
                e.x += dx;
                for s in 0..self.width {
                    self.map.insert((e.y,e.x+s), idx);
                }
            }
            // Move robot
            let ref mut r = &mut self.entities[to_move[0]];
            r.y += dy;
            r.x += dx;
        }
    }

    fn _move(&self, idx: EntityIndex, dir: C, w: isize) -> Option<Vec<usize>> {
        let ref e = self.entities[idx];
        let (dy, dx) = dir;
        if e.movable {
            let mut to_move = vec![idx];
            match dx {
                0 => {
                    for x in 0..w {
                        let p = (e.y + dy, e.x + x);
                        if let Some(jdx) = self.map.get(&p) {
                            if let Some(mut moves) = self._move(*jdx, dir, self.width) {
                                to_move.append(&mut moves);
                            }
                            else {
                                return None
                            }
                        }
                    }
                },
                -1 => {
                    let p = (e.y, e.x - 1);
                    if let Some(jdx) = self.map.get(&p) {
                        if let Some(mut moves) = self._move(*jdx, dir, self.width) {
                            to_move.append(&mut moves);
                        }
                        else {
                            return None
                        }
                    }
                },
                1 => {
                    let p = (e.y, e.x + w);
                    if let Some(jdx) = self.map.get(&p) {
                        if let Some(mut moves) = self._move(*jdx, dir, self.width) {
                            to_move.append(&mut moves);
                        }
                        else {
                            return None
                        }
                    }
                },
                _ => unreachable!()
            }
            return Some(to_move)
        }
        None
    }        
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (ym, xm) = self.max_pos;
        let robot = self.entities.last().unwrap();
        for j in 0..ym {
            let mut line = String::new();
            for i in 0..xm {
                if let Some(idx) = self.map.get(&(j,i)) {
                    if self.entities[*idx].movable {
                        line.push('O');
                    }
                    else {
                        line.push('#');
                    }
                }
                else if robot.y == j && robot.x == i {
                    line.push('@');
                }
                else {
                    line.push('.');
                }
            }
            let _ = write!(f, "{}\n", line);
        }
        Ok(())
    }
}

fn main() {
    let input = read_from_stdin();
    let (map_input, moves) = input.split_once("\n\n").unwrap();

    let mut map_part1 = Map::new(map_input, 1);
    let mut map_part2 = Map::new(map_input, 2);
    //println!("{}", map_part2);

    for m in moves.trim().chars() {
        let dir = match m {
            '^' => (-1,0),
            '<' => (0,-1),
            'v' => (1,0),
            '>' => (0,1),
            _ => {
                continue
            }
        };
        map_part1.move_robot(dir);
        map_part2.move_robot(dir);
        //println!("{}", map_part2);
    }

    let mut gps_sum = 0;
    for e in map_part1.entities.iter().rev().skip(1) {
        if e.movable {
            gps_sum += 100 * e.y + e.x;
        }
    }
    println!("Part 1: {}", gps_sum);

    gps_sum = 0;
    for e in map_part2.entities.iter().rev().skip(1) {
        if e.movable {
            gps_sum += 100 * e.y + e.x;
        }
    }
    println!("Part 2: {}", gps_sum);
}
