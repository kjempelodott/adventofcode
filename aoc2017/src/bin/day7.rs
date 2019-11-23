// Keywords: graph, weights, dfs

#[macro_use]
extern crate adventofcode2017;

use std::collections::{HashMap,HashSet,VecDeque};

struct Node<'a> {
    name: &'a str,
    weight: i32,
    parent: Option<&'a str>,
    children: Vec<&'a str>
}

impl<'a> Node<'a> {
    fn new(name: &'a str, weight: i32) -> Self {
        Node {
            name: name,
            weight: weight,
            parent: None,
            children: vec![],
        }
    }

    fn total_weight(&self, tree: &HashMap<&str, Node>) -> i32 {
        self.weight + self.children.iter()
            .map(|c| ja![tree.get(c)].total_weight(&tree))
            .sum::<i32>()
    }
}

fn solve(input: &str) -> (&str, i32) {    

    let mut tree: HashMap<&str, Node> = HashMap::new();
    for line in input.lines() {
        let mut iter = line.split_whitespace();
        let prog = ja![iter.next()];
        let weight = ja![ja![iter.next()]
                         .trim_matches(|c| c == '(' || c == ')')
                         .parse::<i32>()];
        let mut node = Node::new(prog, weight);
        
        if let Some(_) = iter.next() {
            for disc in iter {
                let disc = disc.trim_matches(',');
                let mut child = tree.entry(disc).or_insert(Node::new(disc, 0));
                child.parent = Some(prog);
                node.children.push(disc);
            }
        }
        if let Some(_node) = tree.get(prog) {
            node.parent = _node.parent;
        }
        tree.insert(prog, node);
    }
    let (root, _) = ja![tree.iter().find(|&(_, n)| n.parent.is_none())];

    
    let mut seen: HashSet<&str> = HashSet::new();
    let mut queue: VecDeque<&Node> = tree
        .iter()
        .filter(|&(_, n)| n.children.len() == 0)
        .map(|(_, n)| n)
        .collect();

    while let Some(node) = queue.pop_back() {
        if seen.contains(node.name) {
            continue
        }
        if let Some(parent) = node.parent {
            let parent = ja![tree.get(parent)];
            parent.children.iter().for_each(|c| { seen.insert(c); } );
            if parent.children.len() > 1 {
                let mut c: Vec<(&Node, i32)> = parent.children
                    .iter()
                    .map(|c| {
                        let node = ja![tree.get(c)];
                        let w = node.total_weight(&tree);
                        (node, w)
                    })
                    .collect();
                c.sort_by_key(|&(_, w)| w);
                let (min, max) = (c[0].1, c[c.len()-1].1);
                if max != min {
                    if c[0].1 == c[1].1 {
                        return (root, c[c.len()-1].0.weight - max + min);
                    }
                    return (root, c[0].0.weight + max - min);
                }
            }
            queue.push_front(parent);
        }
    }
    unreachable!();
}

fn main() {
    let input = include_str!("../../res/day7");
    let (root, diff) = solve(input);
    println!("Part 1: {}", root);
    println!("Part 2: {}", diff);
}
