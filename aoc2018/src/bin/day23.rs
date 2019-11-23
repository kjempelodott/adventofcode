#[macro_use]
extern crate adventofcode2018;
extern crate itertools;
extern crate counter;
use adventofcode2018::read_from_stdin;
use itertools::Itertools;
use counter::Counter;
use std::collections::{HashSet,HashMap};

#[derive(Debug)]
struct Pos(isize, isize, isize);
#[derive(Debug)]
struct Bot {
    pos: Pos,
    range: isize
}

impl Pos {
    fn dist(&self, o: &Pos) -> isize {
        (self.0-o.0).abs() + (self.1-o.1).abs() + (self.2-o.2).abs()
    }
}

impl Bot {
    fn in_range(&self, other: &Pos) -> bool {
        self.dist(&other) <= self.range
    }
    fn dist(&self, o: &Pos) -> isize {
        self.pos.dist(&o)
    }
}

fn main() {
    let input: Vec<String> = split_into!(read_from_stdin());
    let mut bots: Vec<Bot> = input.iter()
        .map(|l| {
            let p = numbers!(l => isize);
            Bot { pos: (Pos(p[0], p[1], p[2])), range: p[3] }
        })
        .collect();

    let max_r_bot = bots.iter().max_by_key(|b| b.range).unwrap();
    let n_in_range = bots.iter()
        .filter(|b| max_r_bot.in_range(&b.pos))
        .count();

    println!("Part 1: {}", n_in_range);
    
    let mut x = bots.iter().map(|b| b.pos.0).minmax().into_option().unwrap();
    let mut y = bots.iter().map(|b| b.pos.1).minmax().into_option().unwrap();
    let mut z = bots.iter().map(|b| b.pos.2).minmax().into_option().unwrap();
    
    let origo = Pos(0,0,0);
    let mut box_size = 2usize.pow(1);
    let mut best = 0;
    while box_size > 1 {
        let d = box_size as isize;
        let mut best_md = 0;
        for i in (x.0..x.1).step_by(box_size) {
            for j in (y.0..y.1).step_by(box_size) {
                for k in (z.0..z.1).step_by(box_size) {
                    let this = Pos(i,j,k);
                    let in_range = bots.iter()
                        .filter(|b| b.dist(&this) < b.range + d)
                        .count();
                    if in_range >= best {
                        let md = origo.dist(&this);
                        if in_range > best || md < best_md {
                            best = in_range;
                            best_md = origo.dist(&this);
                            x = (i-d,i+d);
                            y = (j-d,j+d);
                            z = (k-d,k+d);
                        }
                    }
                }
                println!("{}", box_size);
                println!("{}", best);
                println!("{}", best_md);

            }
        }
        box_size = box_size >> 1;
    }
    
    // Manhattan distance metric
    // Spheres are equivalent to octahedrons in Euclidean space
    
    // let max_in_range = bots.iter().clone()
    //     .map(|ref b|
    //          bots.iter()
    //          .filter(|c| b.pos.dist(&c.pos) <= b.range + c.range)
    //          .count())
    //     .max()
    //     .unwrap();
    // println!("{:?}", max_in_range);
    
    // for b in bots.iter().clone() {
    //     if bots.iter()
    //         .filter(|c| b.pos.dist(&c.pos) <= b.range + c.range)
    //         .count() == max_in_range {
    //             println!("{:?}", b);
    //             println!("{}", b.dist(&Pos(0,0,0,)));
    //         }
    //}

    //too low: 86593605
    
    // let origo = Pos(0,0,0);
    // let mut incr = [
    //     (1,1,1),
    //     (1,1,),
    //     (1,-1,0),
    //     (1,0,1),
    //     (-1,0,1),
    //     (-1,0,-1),
        
    //     (-1,-1,1),
    //     (-1,-1,-1),
    //     (0,-1,0),
    //     (0,0,-1),
    // ]
    // loop {
    //     let n_in_range = bots.iter()
    //         .filter(|b| b.in_range(&me))
    //         .count();
    //     if n_in_range == max_in_range {
    //         println!("Part 2: {}", me.dist(&origo));
    //         break;
    //     }
    //     let d = incr.next().unwrap();
    //     me.0 += d.0;
    //     me.1 += d.1;
    //     me.2 += d.2;
    // }
        
    // let ((x,y,z),r) = density.last().unwrap().1;
    // let mut x = bots.keys().map(|b| b.0).minmax().into_option().unwrap();
    // let mut y = bots.keys().map(|b| b.1).minmax().into_option().unwrap();
    // let mut z = bots.keys().map(|b| b.2).minmax().into_option().unwrap();

    // let mut best = 0;
    // loop {
    //     let dx = (x.1 - x.0).abs();
    //     let dy = (y.1 - y.0).abs();
    //     let dz = (z.1 - z.0).abs();
    //     if dx == 0 && dy == 0 && dz == 0 {
    //         break;
    //     }
    //     let mut best_md = 0;
    //     for &i in &[x.0+dx/4, x.1-dx/4] {
    //         for &j in &[y.0+dy/4, y.1-dy/4] {
    //             for &k in &[z.0+dz/4, z.1-dz/4] {
    //                 let in_range = bots.iter()
    //                     .filter(|b| md((i,j,k), *b.0) <= *b.1)
    //                     .count();
    //                 if in_range > best {
    //                     best = in_range;
    //                     best_md = md((i,j,k), (0,0,0));
    //                     x = (i-dx/4,i+dx/4);
    //                     y = (j-dy/4,j+dy/4);
    //                     z = (k-dz/4,k+dz/4);
    //                 }
    //                 else if in_range == best {
    //                     if md((i,j,k), (0,0,0)) < best_md {
    //                         best = in_range;
    //                         best_md = md((i,j,k), (0,0,0));
    //                         x = (i-dx/4,i+dx/4);
    //                         y = (j-dy/4,j+dy/4);
    //                         z = (k-dz/4,k+dz/4);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     println!("Part 2: {:?} {:?} {:?}", x,y,z);
    //     println!("Part 2: {:?}", best);
    // }
    
    // // (x-r..x+r).cartesian_product(y-r..x+r).cartesian_product(z-r..z+r)
    // //     .max_by_key(|&c| {
    // //         bots.iter()
    // //             .filter(|b| md(((c.0).0,(c.0).1,c.1), *b.0) <= *b.1)
    // //             .count();
    // //     });
    

    
    // let mut div = 10000000;
    // let bot_map = bots.iter()
    //     .flat_map(|b| {
    //         let ((x,y,z),r) = b;
    //         ((x-r)/div..(x+r)/div)
    //             .cartesian_product((y-r)/div..(y+r)/div)
    //             .cartesian_product((z-r)/div..(z+r)/div)
    //     })
    //     .collect::<Counter<((isize,isize),isize)>>();
    // println!("{:?}", bot_map.most_common()[0]);
    // println!("{:?}", bot_map.most_common()[1]);
    // println!("{:?}", bot_map.most_common()[2]);
    // println!("{:?}", bot_map.most_common()[3]);
    // println!("{:?}", bot_map.most_common()[4]);
    // println!("{:?}", bot_map.most_common()[5]);
    // println!("{:?}", bot_map.most_common()[6]);
    

    // let (x0, x1) = bots.keys().map(|b| b.0).minmax().into_option().unwrap();
    // let (y0, y1) = bots.keys().map(|b| b.1).minmax().into_option().unwrap();
    // let (z0, z1) = bots.keys().map(|b| b.2).minmax().into_option().unwrap();
    // (x0..x1).cartesian_product(y0..y1).cartesian_product(z0..z1)
    //     .max_by_key(|&c| {
    //         bots.iter()
    //             .filter(|b| md(((c.0).0,(c.0).1,c.1), *b.0) <= *b.1)
    //             .count();
    //     });
    
    // let (x, y): (Vec<i32>, Vec<i32>) = locs.clone().into_iter().unzip();
    // let (x0, x1) = x.into_iter().minmax().into_option().unwrap();
    // let (y0, y1) = y.into_iter().minmax().into_option().unwrap();


    // // Find largest finite region in Voronoi diagram
    // println!("Part 1: {}", (x0..x1).cartesian_product(y0..y1)
    //          .filter_map(|xy| find_nearest(xy, &locs))
    //          .filter(|xy| finite.contains(xy))
    //          .collect::<Counter<(i32,i32)>>()
    //          .most_common()[0].1);
    // println!("Part 2: {}", (x0..x1).cartesian_product(y0..y1)
    //          .filter(|(x,y)| locs.iter()
    //                  .map(|&xy| md(xy, (*x,*y)))
    //                  .sum::<i32>() < 10000)
    //          .count());
}
