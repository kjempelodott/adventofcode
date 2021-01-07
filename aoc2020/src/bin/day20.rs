#![feature(drain_filter)]
extern crate adventofcode2020;
use adventofcode2020::{into_lines,read_from_stdin};

extern crate itertools;
use itertools::Itertools;

use std::slice::Iter;
use std::collections::HashMap;

/* 
Represent edges as 10-bit numbers
Store N/S and W/E with the same "endianness"

         (3 NORTH)
         ---->-----
         |        |
(2 WEST) V        V (0 EAST)
         |        |
         ---->-----
         (1 SOUTH)

Flipped image:

         (3 NORTH)
         ----<-----
         |        |
(0 EAST) V        V (2 WEST)
         |        |
         ----<-----
         (1 SOUTH)
 */

#[repr(usize)]
#[derive(Debug)]
enum EdgeSide {
    East = 0,
    South = 1,
    West = 2,
    North = 3
}
use EdgeSide::*;

impl From<usize> for EdgeSide {
    fn from(x: usize) -> EdgeSide {
        match x {
            0 => East,
            1 => South,
            2 => West,
            3 => North,
            _ => unreachable!()
        }
    }
}

#[derive(Clone,Debug)]
struct Edge(u16,u16);

impl Edge {
    fn new(s: Vec<bool>) -> Self {
        let edge = s.iter()
            .enumerate()
            .filter(|(_,x)| **x).map(|(i,_)| (1 << i))
            .sum::<u16>();
        Edge(edge, edge.reverse_bits() >> 6)
    }

    fn rev(&mut self) {
        let tmp = self.0;
        self.0 = self.1;
        self.1 = tmp;
    }
}

#[derive(Clone,Debug)]
enum Orientation {
    Identity(bool),
    Rot90(bool),
    Rot180(bool),
    Rot270(bool)
}
use Orientation::*;

impl Orientation {
    fn iter() -> Iter<'static, Orientation> {
        static ORIENTATIONS: [Orientation; 8] = [
            Identity(false),Rot90(false),Rot180(false),Rot270(false),
            Identity(true),Rot90(true),Rot180(true),Rot270(true)
        ];
        ORIENTATIONS.iter()
    }
    fn apply(&self, x: &[Edge;4]) -> [Edge;4] {
        let flip = |e: &mut [Edge;4]| {
            e.swap(0,2);
            e[1].rev();
            e[3].rev();
        };
        match self {
            Identity(f) => {
                let mut e = x.clone();
                if *f { flip(&mut e) }
                e
            },
            Rot90(f) => {
                let mut e = x.clone();
                if *f { flip(&mut e) }
                e.rotate_left(1);
                e[0].rev();
                e[2].rev();
                e
            },
            Rot180(f) => {
                let mut e = x.clone();
                if *f { flip(&mut e) }
                e.rotate_left(2);
                e.iter_mut().for_each(|y| y.rev());
                e
            },
            Rot270(f) => {
                let mut e = x.clone();
                if *f { flip(&mut e) }
                e.rotate_right(1);
                e[1].rev();
                e[3].rev();
                e
            },
        }
    }
}

#[derive(Clone,Debug)]
struct Tile {
    edge: [Edge;4],
    data: Vec<Vec<bool>>,
    adj: Vec<usize>,
}

impl Tile {
    fn parse(image: &[String]) -> (usize, Self) {
        let id = image[0].split(' ')
            .nth(1).unwrap()
            .trim_matches(':').parse()
            .unwrap();

        let data = image[1..11].iter()
            .map(|s| s.chars()
                 .map(|c| c == '#')
                 .collect::<Vec<bool>>())
            .collect::<Vec<_>>();

        let e = data.iter().cloned().map(|v| v[9]).collect::<Vec<_>>();
        let s = data[9].clone();
        let w = data.iter().cloned().map(|v| v[0]).collect::<Vec<_>>();
        let n = data[0].clone();       
        let edge = [Edge::new(e), Edge::new(s), Edge::new(w), Edge::new(n)];

        let tile = Tile {
            edge: edge,
            data: data,
            adj: vec![]
        };
        (id, tile)
    }

    fn align(orientation: &Orientation, data: &mut Vec<Vec<bool>>) {
        let h = data.len();
        let w = data[0].len();
        match orientation {
            Rot90(f) => {
                if *f { data.iter_mut().for_each(|row| row.reverse()) }
                *data = (0..w).rev()
                    .map(|x| (0..h).map(|y| data[y][x]).collect::<Vec<bool>>())
                    .collect();
            },
            Rot180(f) => {
                if *f { data.iter_mut().for_each(|row| row.reverse()) }
                *data = (0..h).rev()
                    .map(|y| (0..w).rev().map(|x| data[y][x]).collect::<Vec<bool>>())
                    .collect();
            },
            Rot270(f) => {
                if *f { data.iter_mut().for_each(|row| row.reverse()) }
                *data = (0..w)
                    .map(|x| (0..h).rev().map(|y| data[y][x]).collect::<Vec<bool>>())
                    .collect();
            },
            Identity(f) => if *f { data.iter_mut().for_each(|row| row.reverse()) }
        }
    }

    #[allow(non_snake_case)]
    fn is_adjacent(&self, other: &Self) -> bool {
        for Edge(a,A) in &self.edge {
            for Edge(b,_) in &other.edge {
                if a == b || A == b { // Implies A == B || a == B
                    return true
                }
            }
        }
        false
    }
}

fn main() {
    let mut tiles = into_lines(read_from_stdin())
        .chunks(11)
        .map(|image| Tile::parse(image))
        .collect::<HashMap<usize,Tile>>();

    let tile_ids = tiles.keys().cloned().collect::<Vec<_>>();
    for &i in tile_ids.iter() {
        let tile = tiles.get(&i).unwrap();
        let mut adj = vec![];
        for &j in tile_ids.iter().filter(|&&j| j != i) {
            let other = tiles.get(&j).unwrap();
            if tile.is_adjacent(&other) {
                adj.push(j);
            }
        }
        tiles.get_mut(&i).unwrap().adj = adj; 
   }

    println!("Part 1: {}", tiles.iter()
             .filter(|(_,t)| t.adj.len() == 2)
             .map(|(t,_)| t)
             .product::<usize>());

    let seed = tile_ids[0];
    let mut placed = HashMap::new();
    placed.insert(seed, (0i8, 0i8));
    let mut queue = tiles.get(&seed).unwrap().adj.clone();

    while let Some(t) = queue.pop() {
        if placed.contains_key(&t) { continue }
        let mut tile = tiles.get(&t).unwrap().clone();
        let (fixed, free): (Vec<_>,Vec<_>) = tile.adj.iter()
            .cloned()
            .partition(|x| placed.contains_key(x));

        for op in Orientation::iter() {
            let edge = op.apply(&tile.edge);
            let mut m = vec![];
            for f in fixed.iter() {
                let ft = tiles.get(f).unwrap();
                for (i,e) in ft.edge.iter().enumerate() {
                    if e.0 == edge[(i + 2) % 4].0 {
                        let &(y,x) = placed.get(f).unwrap();
                        m.push(match EdgeSide::from(i) {
                            East => (y, x+1),
                            North => (y-1, x),
                            West => (y, x-1),
                            South => (y+1, x)
                        });
                        break
                    }
                }
            }
            if m.len() == fixed.len() {
                let z = m.iter().fold((0,0), |p,z| (p.0 + z.0, p.1 + z.1));
                let p = (z.0 / m.len() as i8, z.1 / m.len() as i8);
                Tile::align(op, &mut tile.data);
                tile.edge = edge;
                tiles.insert(t, tile);
                placed.insert(t, (p.0, p.1));
                break
            }
        }
        free.iter().for_each(|&t| queue.push(t));
    }

    let (&y0, &y1) = placed.values().map(|(y,_)| y).minmax().into_option().unwrap();
    let (&x0, &x1) = placed.values().map(|(_,x)| x).minmax().into_option().unwrap();
    let imh = 8 * (1 + y1 - y0) as usize;
    let imw = 8 * (1 + x1 - x0) as usize;
    let mut image = vec![vec![false;imh];imw];
    for (t,(y,x)) in placed.iter() {
        let tile = &tiles.get(t).unwrap().data;
        let j = 8 * (y - y0) as usize;
        let i = 8 * (x - x0) as usize;
        for k in 0..8 {
            image[j+k][i..i+8].copy_from_slice(&tile[k+1][1..9])
        }
    }
    let monster = ["                  # ",
                   "#    ##    ##    ###",
                   " #  #  #  #  #  #   "]
        .iter()
        .map(|s| s.chars()
             .map(|c| c == '#')
             .collect::<Vec<bool>>())
        .collect::<Vec<_>>();

    for op in Orientation::iter() {
        let mut mon = monster.clone();
        Tile::align(op, &mut mon);
        let mon_h = mon.len();
        let mon_w = mon[0].len();
        let monsters = (0..(imh - mon_h))
            .map(|j| (0..(imw - mon_w))
                 .filter(|i| (0..mon_h)
                         .all(|k| (0..mon_w)
                              .all(|l| !mon[k][l] || image[j+k][i+l])
                            )
                 ).count()
            ).sum::<usize>();
        if monsters > 0 {
            println!("Part 2: {}", image.iter()
                     .map(|v| v.iter().filter(|&&w| w).count())
                     .sum::<usize>() - monsters * 15);
            break
        }
    }
}
