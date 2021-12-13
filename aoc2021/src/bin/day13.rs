extern crate aoc2021;
use aoc2021::read_from_stdin;

extern crate parse_display;
use parse_display::{Display,FromStr};

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
            Fold::X(pos) => if p.0 > *pos {
                p.0 = 2*pos - p.0;
            },
            Fold::Y(pos) => if p.1 > *pos {
                p.1 = 2*pos - p.1;
            }
        }
        p
    }
}

fn main() {
    let input = read_from_stdin();
    let (dots, folds) = input.split_once("\n\n").unwrap();
    let d: Vec<P> = dots.lines()
        .map(|l| l.split_once(',').map(|(x,y)| (x.parse().unwrap(), y.parse().unwrap())).unwrap())
        .collect();
    let f: Vec<Fold> = folds.lines()
        .map(|l| l.parse().unwrap())
        .collect();
    println!("Part 1: {}", d.iter().map(|&p| f[0].fold(p))
             .collect::<Vec<P>>()
             .len());

    let sheet: Vec<P> = d.iter()
        .map(|&p| f.iter().fold(p, |a, x| x.fold(a)))
        .collect();
    let y1 = sheet.iter().max_by_key(|p| p.1).unwrap().1;
    let x1 = sheet.iter().max_by_key(|p| p.0).unwrap().0;
    for y in 0..y1+1 {
        println!("{}", (0..x1+1).map(|x| if sheet.contains(&(x,y)) { '#' } else { ' ' })
                 .collect::<String>());
    }
}
