use std::collections::{BTreeMap,HashSet,BinaryHeap};
use aoc2024::{read_from_stdin};

type Node = (char,char);

#[derive(Clone,Default)]
struct Graph {
    stars: BTreeMap<Node,Vec<Node>>
}

impl Graph {
    fn add_edge(&mut self, node: Node, other: Node) {
        self.stars.entry(node).or_default().push(other);
        self.stars.entry(other).or_default().push(node);
    }

    fn get_3complete(&self) -> HashSet<Vec<Node>> {
        let mut complete3: HashSet<Vec<Node>> = HashSet::new();
        for (n, nn) in self.stars.iter() {
            for m in nn.iter() {
                if let Some(mn) = self.stars.get(&m) {
                    for p in mn.iter() {
                        if let Some(pn) = self.stars.get(&p) {
                            if pn.contains(n) {
                                let mut v = vec![*n,*m,*p];
                                v.sort();
                                complete3.insert(v);
                            }
                        }
                    }
                }
            }
        }
        complete3
    }

    fn get_max_complete(&self) -> HashSet<Node> {
        let mut max_complete: HashSet<Node> = HashSet::new();
        for n in self.stars.keys() {
            let mut complete: HashSet<Node> = HashSet::new();
            complete.insert(*n);

            let mut heap = BinaryHeap::new();
            heap.push(n);

            let mut visited: HashSet<Node> = HashSet::new();
            'outer: while let Some(n) = heap.pop() {
                if visited.contains(&n) {
                    continue;
                }
                visited.insert(*n);
                if let Some(nn) = self.stars.get(&n) {                
                    for m in nn.iter() {
                        if let Some(mm) = self.stars.get(&m) {
                            for x in complete.iter() {
                                if !mm.contains(&x) {
                                    continue 'outer
                                }
                            }
                        }
                        complete.insert(*m);
                        heap.push(m);                
                    }
                }
            }
            if complete.len() > max_complete.len() {
                max_complete = complete;
            }
        }
        max_complete
    }
}

fn main() {
    let mut graph = Graph::default();
    for line in read_from_stdin().lines() {
        let c = line.chars().collect::<Vec<char>>();
        let n1 = (c[0], c[1]); 
        let n2 = (c[3], c[4]);
        graph.add_edge(n1, n2);
    }

    let c3 = graph.get_3complete();
    println!("Part 1: {}", c3.iter().filter(|t| t.iter().any(|n| n.0 == 't')).count());

    let comp = graph.get_max_complete();
    let mut v = comp.iter()
        .map(|n| {
            let mut s = String::new();
            s.push(n.0);
            s.push(n.1);
            s
        })
        .collect::<Vec<String>>();
    v.sort();
    println!("Part 2: {}", v.join(","));
}
