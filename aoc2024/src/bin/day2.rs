use itertools::Itertools;
use aoc2024::{numbers,read_from_stdin};

fn cmp(input: impl Iterator<Item = isize> + Clone) -> bool {
    (input.clone().all(|d| (d > 0)) || input.clone().all(|d| (d < 0))) && input.clone().all(|d| isize::abs(d) < 4)
}

fn main() {
    let reports = read_from_stdin()
        .lines()
        .map(|line| numbers!(line => isize))
        .collect::<Vec<Vec<isize>>>();

    let mut part1 = 0;
    let mut part2 = 0;
    'outer: for report in reports.iter() {
        let diffs = report.iter().tuple_windows().map(|(a,b)| a - b).collect::<Vec<_>>();
        if cmp(diffs.iter().cloned()) {
            part1 += 1;
            part2 += 1;
            continue;
        }
        for i in 0..report.len() {
            let mut try_report = report.clone();
            try_report.remove(i);
            let diffs = try_report.iter().tuple_windows().map(|(a,b)| a - b).collect::<Vec<_>>();
            if cmp(diffs.iter().cloned()) {
                part2 += 1;
                continue 'outer;
            }
        }
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
