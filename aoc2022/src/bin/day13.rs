extern crate aoc2022;
use aoc2022::read_from_stdin;

#[derive(Clone,PartialEq,Eq,Ord,Debug)]
enum Signal {
    List(Vec<Signal>),
    Integer(u8)
}

impl Signal {
    fn parse(substr: &str) -> Self {
        // Split on commas
        match substr.chars().nth(0) {
            Some('[') => { // New frame
                let substr = &substr[1..substr.len()-1];
                let mut list = vec![];
                if substr.len() == 0 {
                    return Signal::List(list);
                }
                let mut frame = 0; // The list may contain nested lists
                let mut cur = 0; // Position of last comma
                for (i, c) in substr.chars().enumerate() {
                    match c {
                        '[' => { frame += 1; },
                        ']' => { frame -= 1; },
                        ',' if frame == 0 => { // Consume the previous signal
                                list.push(Self::parse(&substr[cur..i]));
                                cur = i + 1;
                        },
                        _ => {},
                    }
                }
                list.push(Self::parse(&substr[cur..])); // Consume the last signal
                Signal::List(list)
            }
            Some(_) => Signal::Integer(substr.trim_end_matches(']').parse().unwrap()), // This is a single integer value
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Signal {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        match (self, rhs) {
            (Signal::Integer(v), Signal::Integer(u)) => v.partial_cmp(u),
            (Signal::List(l), Signal::List(k)) => l.partial_cmp(k),
            (Signal::List(l), Signal::Integer(u))=> l.partial_cmp(&vec![Signal::Integer(*u)]),
            (Signal::Integer(v), Signal::List(k)) => vec![Signal::Integer(*v)].partial_cmp(k),
        }
    }
}

fn main() {
    let mut packets: Vec<_> = read_from_stdin().lines().filter_map(|line| {
        if line.is_empty() {
            return None
        }
        Some(Signal::parse(line))
    }).collect();

    println!("Part 1: {}",  packets.chunks(2).enumerate().filter_map(|(i,pair)| {
        if pair[0] < pair[1] {
            return Some(i + 1)
        }
        None
    }).sum::<usize>());
    let l2 = Signal::parse("[[2]]");
    let l6 = Signal::parse("[[6]]");
    packets.push(l2.clone());
    packets.push(l6.clone());
    packets.sort();
    println!("Part 2: {}", (1 + packets.iter().position(|s| *s == l2).unwrap()) * (1 + packets.iter().position(|s| *s == l6).unwrap()));
}
