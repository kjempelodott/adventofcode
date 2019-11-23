extern crate adventofcode2018;
use adventofcode2018::{into_lines,read_from_stdin};
use std::collections::{HashMap,HashSet};

fn main() {
    let input = into_lines(read_from_stdin());
    let mut state: String = input[0].split(": ").nth(1).unwrap().into();
    let mut seen: HashSet<String> = HashSet::new();
    seen.insert(state.trim_matches('.').into());
    let rules: HashMap<String,u8> = input[1..].iter().map(|l| {
        let mut split = l.split(" => ");
        (split.next().unwrap().into(), split.next().unwrap().as_bytes()[0])
    }).collect();

    let mut offset = 4;
    state = "....".to_owned() + &state + &"....".to_owned();
    for i in 0.. {
        if i == 20 {
            println!("Part 1: {}", state.chars()
                     .enumerate()
                     .filter(|(_,c)| *c == '#')
                     .map(|(j,_)| j - offset)
                     .sum::<usize>());
        }
        let mut tmp = state.clone().into_bytes();
        for (rule,res) in rules.iter() {
            for j in 0..tmp.len()-5 {
                if &state[j..j+5] == rule {
                    tmp[j+2] = *res;
                }
            }
        }
        state = String::from_utf8_lossy(&tmp).into();
        if !state.starts_with("....") {
            state = "....".to_owned() + &state;
            offset += 4;
        }
        if !state.ends_with("....") {
            state = state + &"....".to_owned();
        }
        if seen.insert(state.trim_matches('.').into()) == false {
            println!("Part 2: {}", state.chars()
                     .enumerate()
                     .filter(|(_,c)| *c == '#')
                     .map(|(j,_)| j - offset + 50000000000 - i - 1)
                     .sum::<usize>());
            break
        }
    }
}
