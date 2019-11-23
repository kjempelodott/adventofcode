// Keywords: scanner

use std::collections::HashMap;

fn severity(firewall: &HashMap<u32, u32>, delay: u32, last: u32) -> Option<u32> {
    let is_caught = |p, v| (p + delay) % (2 * (v - 1)) == 0;
    let caught: Vec<u32> = (0..last)
        .filter_map(|p| firewall.get(&p).map(|v| (p, v)))
        .filter_map(|(p, v)| if is_caught(p, v) { Some(p * v) } else { None })
        .collect();
    if caught.is_empty() {
        return None;
    }
    Some(caught.iter().sum())
}

fn solve(input: &str) -> (u32, u32) {
    let firewall: HashMap<u32, u32> = input.lines()
        .map(|l| l.split(": ")
             .map(|n| n.parse::<u32>().unwrap())
             .collect::<Vec<u32>>())
        .map(|v| (v[0], v[1]))
        .collect();
    let last = firewall.keys().max().unwrap() + 1;
    let sev = severity(&firewall, 0, last).unwrap_or(0);

    for delay in 1.. {
        if severity(&firewall, delay, last).is_none() {
            return (sev, delay);
        }
    }
    unreachable!();
}   
    
fn main() {
    let input = include_str!("../../res/day13");
    let (severity, delay) = solve(input);
    println!("Part 1: {}", severity);
    println!("Part 2: {}", delay);
}
