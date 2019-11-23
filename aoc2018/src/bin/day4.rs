extern crate adventofcode2018;
extern crate counter;
use adventofcode2018::{into_lines,read_from_stdin};
use counter::Counter;
use std::collections::HashMap;

fn main() {
    let mut records = into_lines(read_from_stdin());
    records.sort_unstable();
    records.reverse();

    let mut guards: HashMap<usize, Counter<usize>> = HashMap::new();
    while records.len() > 0 {
        let mut record = records.pop().unwrap();
        let guard = record.split_whitespace()
            .nth(3)
            .and_then(|e| e.get(1..))
            .and_then(|e| e.parse::<usize>().ok())
            .unwrap();

        let mut min0 = 0;
        let mut awake = true;
        while records.last().map_or(false, |r| !r.contains("Guard")) {
            record = records.pop().unwrap();
            let min1 = record.split_whitespace()
                .nth(1)
                .and_then(|e| e.get(3..5))
                .and_then(|e| e.parse::<usize>().ok())
                .unwrap();
            if !awake {
                guards.entry(guard.clone())
                    .or_default()
                    .update(min0..min1);
            }
            awake ^= true;
            min0 = min1
        }
    }

    let max_sleep_guard = guards.iter()
        .max_by_key(|(_,r)| r.values().sum::<usize>())
        .unwrap().0;
    let max_min = guards.get(max_sleep_guard).unwrap().most_common();
    println!("Part 1: {}", max_sleep_guard * max_min[0].0);

    let max_sleep = guards.iter()
        .map(|(g,r)| {
            let max_min = r.most_common()[0];
            (g, max_min.0, max_min.1)
        })
        .max_by_key(|(_,_,s)| s.clone())
        .unwrap();
    println!("Part 2: {}", max_sleep.0 * max_sleep.1);
}
