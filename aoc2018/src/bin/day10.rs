extern crate adventofcode2018;
extern crate itertools;
use adventofcode2018::{into_lines,read_from_stdin};
use itertools::Itertools;

fn main() {
    let (mut xy, dxy): (Vec<_>, Vec<_>) = into_lines(read_from_stdin())
        .iter()
        .map(|l| {
            l.split(|c: char| c == '>' || c == '<' || c == ',' || c.is_whitespace())
                .filter_map(|d| d.parse::<isize>().ok())
                .collect::<Vec<isize>>()
        })
        .map(|v| ((v[0], v[1]), (v[2], v[3])))
        .unzip();

    let mut ym = xy.iter().map(|p| p.1).minmax().into_option().unwrap();
    let mut wait = 0;
    while ym.1 - ym.0 >= 10 {
        xy.iter_mut().zip(dxy.iter())
            .for_each(|(p,&v)| { p.0 += v.0; p.1 += v.1; });
        ym = xy.iter().map(|v| v.1).minmax().into_option().unwrap();
        wait += 1;
    }

    let xm = xy.iter().map(|&p| p.0).minmax().into_option().unwrap();
    xy.iter_mut().for_each(|p| { p.0 -= xm.0; p.1 -= ym.0; });
    println!("Part 1:");
    for r in 0..ym.1-ym.0+1 {
        println!("{}", (0..xm.1-xm.0+1).map(|c| {
            if xy.contains(&(c as isize, r as isize)) { '\u{2588}' }
            else { ' ' }
        }).collect::<String>());
    }
    println!("Part 2: {}", wait);
}
