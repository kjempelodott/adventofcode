#![feature(str_split_once)]

#[macro_use]
extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;

#[derive(Clone)]
enum Match {
    Unset,
    Single(u8),
    Substr(Vec<usize>),
    Either(Vec<usize>,Vec<usize>)
}

use Match::*;

fn capture(msg: &[u8], pos: usize, r: &Match, rules: &Vec<Match>) -> Option<Vec<usize>> {
    match r {
        Single(ab) => {
            if msg.get(pos) == Some(ab) {
                return Some(vec![pos+1]) // Matched exactly one
            }
            None
        },
        // For each submatch, there may be more than one match.
        // In order follow each and one of them, we have to
        // recurse further on each submatch.
        Substr(substr) => {
            let ridx = substr[0];
            let match_pos = capture(&msg, pos, &rules[ridx], &rules);
            // The last of a probably larger Substr
            if substr.len() == 1 {
                return match_pos
            }
            let mut result = vec![];
            // Follow each match and try to match the remaining substring
            if let Some(match_pos) = match_pos {
                let subsubstr = Substr(substr[1..].to_vec());
                for p in match_pos {
                    if let Some(next_match_pos) = capture(&msg, p, &subsubstr, &rules) {
                        result.extend(next_match_pos);
                    }
                }
            }
            if result.len() > 0 {
                return Some(result);
            }
            None
        }
        Either(b,d) => {
            let v = capture(&msg, pos, &Substr(b.clone()), &rules);
            let w = capture(&msg, pos, &Substr(d.clone()), &rules);
            if w.is_none() {
                return v;
            }
            if v.is_none() {
                return w;
            }
            let mut z = v.unwrap();
            z.extend(w.unwrap());
            Some(z)
        },
        Unset => unreachable!()
    }
}

fn main() {
    let input = read_from_stdin();
    let mut rules = vec![Unset;131];
    for l in input.lines().take(131) {
        let (rid, rule) = l.split_once(':').unwrap();
        let idx = rid.parse::<usize>().unwrap();
        if rule.contains('|') {
            let (v,w) = rule.split_once('|').unwrap();
            rules[idx] = Either(
                numbers!(v => usize),
                numbers!(w => usize)
            );
        }
        else {
            let n = numbers!(rule => usize);
            rules[idx] = match n.len() {
                0 => Single(rule.as_bytes()[2]),
                1..=3 => Substr(n),
                _ => unreachable!()
            };
        }
    }

    println!("Part 1: {}", input.lines().skip(132)
             .filter(|msg| capture(&msg.as_bytes(), 0, &rules[0], &rules)
                     .map_or(false, |v| v.contains(&msg.len())))
             .count());
    rules[8] = Either(vec![42],vec![42, 8]);
    rules[11] = Either(vec![42, 31],vec![42,11,31]);
    println!("Part 2: {}", input.lines().skip(132)
             .filter(|msg| capture(&msg.as_bytes(), 0, &rules[0], &rules)
                     .map_or(false, |v| v.contains(&msg.len())))
             .count());
}
