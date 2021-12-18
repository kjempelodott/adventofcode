extern crate aoc2021;
use aoc2021::read_from_stdin;

fn eval<'a>(p: &'a [u8], n: usize) -> usize {
    (0..n).map(|i| (p[i] << (n-1-i)) as usize).sum()
}

fn parse_packet<'a>(p: &'a [u8]) -> (usize, usize, usize) {
    let version = eval(p, 3);
    let type_id = eval(&p[3..], 3);
    match type_id {
        4 => {
            let mut o = 6;
            let mut n: Vec<usize> = vec![];
            while p[o] != 0 {
                n.push(eval(&p[o+1..], 4));
                o += 5;
            }
            n.push(eval(&p[o+1..], 4));
            let s = (0..n.len()).map(|i| (n[i] << (4*(n.len()-1-i))) as usize).sum();
            (version,s,o+5)
        },
        z => {
            let mut sub = vec![];
            let mut o;
            match p[6] {
                0 => {
                    o = 22;
                    while o < eval(&p[7..], 15) + 22 {
                        let (v,s,l) = parse_packet(&p[o..]);
                        sub.push((v,s));
                        o += l;
                    }
                },
                _ => {
                    o = 18;
                    for _ in 0..eval(&p[7..], 11) {
                        let (v,s,l) = parse_packet(&p[o..]);
                        sub.push((v,s));
                        o += l;
                    }
                }
            };
            let value = match z {
                0 => sub.iter().map(|(_,s)| *s).sum(),
                1 => sub.iter().map(|(_,s)| *s).product(),
                2 => sub.iter().map(|(_,s)| *s).min().unwrap(),
                3 => sub.iter().map(|(_,s)| *s).max().unwrap(),
                5 => if sub[0].1 > sub[1].1  { 1 } else { 0 },
                6 => if sub[0].1 < sub[1].1  { 1 } else { 0 },
                7 => if sub[0].1 == sub[1].1 { 1 } else { 0 },
                _ => unreachable!()
            };
            (sub.iter().map(|(v,_)| *v).sum::<usize>()+version,value,o)
        }
    }
}

fn main() {
    let input = read_from_stdin();
    let data: Vec<u8> = (0..input.len())
        .filter_map(|i| u8::from_str_radix(&input[i..i+1], 16).ok())
        .flat_map(|n| (0..4).rev().map(move |s| (n & (1 << s)) >> s))
        .collect();
    let (version, value, _) = parse_packet(&data);
    println!("Part 1: {}", version);
    println!("Part 2: {}", value);
}
