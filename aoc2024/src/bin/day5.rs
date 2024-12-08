use std::cmp::Ordering;
use std::collections::HashMap;
use aoc2024::{numbers,read_from_stdin};

#[derive(Clone,Debug,Default,Eq)]
struct Page {
    value: isize,
    less_than: Vec<isize>
}

impl Page {
    fn new(num: isize) -> Self {
        Page { value: num, less_than: vec![] }
    }
    fn add_rule(&mut self, num: isize) {
        self.less_than.push(num);
    }
}

impl PartialEq for Page {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Ord for Page {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.less_than.iter().any(|&v| other.value == v) {
            return Ordering::Less;
        }
        if other.less_than.iter().any(|&v| self.value == v) {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
}

impl PartialOrd for Page {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = read_from_stdin();
    let (page_ordering_rules, updates) = input.split_once("\n\n").unwrap();

    let mut pages: HashMap<isize,Page> = HashMap::new();
    for line in page_ordering_rules.lines() {
        let n: Vec<isize> = numbers!(line => isize);
        pages.entry(n[0]).or_insert(Page::new(n[0])).add_rule(n[1]);
        pages.entry(n[1]).or_insert(Page::new(n[1]));
    }

    let mut part1 = 0;
    let mut part2 = 0;
    for line in updates.lines() {
        let pn: Vec<isize> = numbers!(line => isize);
        let mut pvec = pn.iter()
            .map(|p| pages.get(p).unwrap().clone())
            .collect::<Vec<Page>>();
        let n = pvec.len();
        if pvec.is_sorted() {
            part1 += pvec[n - n.div_ceil(2)].value;
        }
        else {
            pvec.sort();
            part2 += pvec[n - n.div_ceil(2)].value;
        }
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
