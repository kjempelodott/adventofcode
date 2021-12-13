#[macro_use]
extern crate aoc2021;
use aoc2021::read_from_stdin;

extern crate parse_display;
use parse_display::{Display,FromStr};
use std::collections::HashSet;

type P = (usize, usize);

#[derive(Debug,Display,FromStr,PartialEq)]
#[display("fold along {}={0}")]
#[display(style="lowercase")]
enum Fold {
    X(usize),
    Y(usize)
}

impl Fold {
    fn fold(&self, mut p: P) -> P {
        match self {
            Fold::X(pos) => {
                if p.0 > *pos {
                    p.0 = 2*pos - p.0;
                }
            },
            Fold::Y(pos) => {
                if p.1 > *pos {
                    p.1 = 2*pos - p.1;
                }
            }
        }
        p
    }
}

fn main() {
    let input = read_from_stdin();
    let (dots, folds) = input.split_once("\n\n").unwrap();
    let d: HashSet<P> = dots.lines().map(|l| {
        let (x,y) = l.split_once(',').unwrap();
        (x.parse().unwrap(), y.parse().unwrap())
    }).collect();
    let f: Vec<Fold> = folds.lines()
        .map(|l| l.parse().unwrap())
        .collect();
    println!("Part 1: {}", d.iter().map(|&p| f[0].fold(p))
             .collect::<HashSet<P>>()
             .len());

    let mut sheet: HashSet<P> = d.iter()
        .map(|&p| f.iter().fold(p, |a, x| x.fold(a)))
        .collect();
    let ymax = sheet.iter().max_by_key(|p| p.1).unwrap().1;
    let ymin = sheet.iter().min_by_key(|p| p.1).unwrap().1;
    let xmax = sheet.iter().max_by_key(|p| p.0).unwrap().0;
    let xmin = sheet.iter().min_by_key(|p| p.0).unwrap().0;
    for y in ymin..ymax+1 {
        println!("{}", (xmin..xmax+1).map(|x| sheet.get(&(x,y)).map_or(' ', |_| '#'))
                 .collect::<String>());
    }
}
