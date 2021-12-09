extern crate aoc2021;
use aoc2021::read_from_stdin;

use std::collections::{HashMap,HashSet};

fn main() {
    let segments = "abcdefg".chars().collect::<HashSet<char>>();
    let mut count_part1 = 0;
    let mut sum_part2 = 0;

    for line in read_from_stdin().lines() {
        let mut patterns = vec![];
        for pattern in line.split(&[' ', '|'][..]).filter(|p| ! p.is_empty()) {
            let mut pattern = pattern.chars().collect::<Vec<char>>();
            pattern.sort();
            patterns.push(pattern.iter().collect::<String>());
        }

        //  6666
        // 5    4 
        // 5    4
        //  3333
        // 2    1
        // 2    1
        //  0000

        let mut search = (0..7).map(|_| segments.clone()).collect::<Vec<_>>();
        for pattern in patterns.iter() {
            match pattern.len() {
                2 => { // 1
                    for i in [0,2,3,5,6] {
                        pattern.chars().for_each(|c| { search[i].remove(&c); });
                    }
                    for i in [1,4] {
                        search[i].retain(|&c| pattern.contains(c));
                    }
                },
                3 => { // 7
                    for i in [0,2,3,5] {
                        pattern.chars().for_each(|c| { search[i].remove(&c); });
                    }
                    for i in [1,4,6] {
                        search[i].retain(|&c| pattern.contains(c));
                    }
                },
                4 => { // 4
                    for i in [0,2,6] {
                        pattern.chars().for_each(|c| { search[i].remove(&c); });
                    }
                    for i in [1,3,4,5] {
                        search[i].retain(|&c| pattern.contains(c));
                    }
                }
                5 => { // 2,3,5
                    for i in [0,3,6] {
                        search[i].retain(|&c| pattern.contains(c));
                    }
                },
                6 => { // 0,6,9
                    for i in [0,1,5,6] {
                        search[i].retain(|&c| pattern.contains(c));
                    }
                },
                _ => {}
            };
        }
        let _tmp_found = search.iter()
            .filter(|s| s.len() == 1)
            .map(|s| s.iter().cloned().next().unwrap())
            .collect::<Vec<_>>();
        for i in 0..7 {
            if search[i].len() > 1 {
                _tmp_found.iter().for_each(|c| { search[i].remove(&c); });
            }
        }
        let found = search.iter()
            .map(|s| s.iter().cloned().next().unwrap())
            .collect::<Vec<_>>();

        let mut wiring: HashMap<String,usize> = HashMap::new();
        let digits = [
            vec![0,1,2,4,5,6],
            vec![1,4],
            vec![0,2,3,4,6],
            vec![0,1,3,4,6],
            vec![1,3,4,5],
            vec![0,1,3,5,6],
            vec![0,1,2,3,5,6],
            vec![1,4,6],
            vec![0,1,2,3,4,5,6],
            vec![0,1,3,4,5,6]
        ];
        for (i,d) in digits.iter().enumerate() {
            let mut wire = d.iter().map(|&i| found[i]).collect::<Vec<char>>();
            wire.sort();
            wiring.insert(wire.iter().collect::<String>(), i);
        }
        count_part1 += patterns.iter()
            .skip(10)
            .filter(|&p| [1,4,7,8].contains(wiring.get(p).unwrap())).count();
        sum_part2 += patterns.iter()
            .skip(10)
            .zip([1000,100,10,1])
            .map(|(p,m)| *wiring.get(p).unwrap() * m)
            .sum::<usize>();
    }
    println!("Part 1: {}", count_part1);
    println!("Part 2: {}", sum_part2);
}
