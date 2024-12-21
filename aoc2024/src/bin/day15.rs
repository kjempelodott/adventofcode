use std::collections::HashMap;
use aoc2024::read_from_stdin;

type C = (isize, isize);

#[derive(Clone, Copy)]
enum Entity {
    Robot,
    Box,
    Wall
}

use Entity::*;

fn can_move(c: C, e: Entity, map: &HashMap<C,Entity>, dir: C) -> bool {
    match e {
        Wall => false,
        Box|Robot => {
            let p = (c.0 + dir.0, c.1 + dir.1);
            match map.get(&p) {
                None => true,
                Some(t) => {
                    match t {
                        Wall => false,
                        Box => can_move(p, *t, map, dir),
                        Robot => unreachable!()
                    }
                }
            }
        }
    }
}

fn main() {
    let input = read_from_stdin();
    let (map_input, moves) = input.split_once("\n\n").unwrap();
    let mut map = HashMap::new(); 
    let mut robot = C::default();

    for (j,line) in map_input.lines().enumerate() {
        for (i,c) in line.chars().enumerate() {
            let i = i as isize;
            let j = j as isize;
            match c {
                '#' => {
                    map.insert((j, i), Wall);
                }
                'O' => {
                    map.insert((j, i), Box);
                },
                '@' => {
                    robot = (j, i);
                },                
                _ => {}
            }
        }
    }

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
        if can_move(robot, Robot, &map, dir) {
            let mut moved = HashMap::new();
            let mut next = (robot.0 + dir.0, robot.1 + dir.1);
            robot = next;
            while let Some(adj) = map.remove(&next) {
                next = (next.0 + dir.0, next.1 + dir.1);
                moved.insert(next, adj);
            }
            for (c,e) in moved.iter() {
                map.insert(*c,*e);
            }
        }
    }

    let mut gps_sum = 0;
    for (c,e) in map.iter() {
        match e {
            Box => {
                gps_sum += 100 * c.0 + c.1
            },
            _ => {}
        }
    }
    println!("{}", gps_sum);
}
