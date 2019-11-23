#[macro_use]
extern crate adventofcode2018;
extern crate itertools;
use adventofcode2018::{into_lines,read_from_stdin};
use itertools::Itertools;
use std::collections::{HashSet,HashMap};

type R = [usize; 4]; // Registers
type Op = Box<Fn(R, usize, usize) -> usize>;

fn main() {
    let mut input = read_from_stdin();
    let offset = input.find("\n\n\n").unwrap();
    let program = input.split_off(offset);
    let samples = into_lines(input);

    // Operations
    let ops: Vec<Op> = vec![
        Box::new(|r,a,b| r[a] + r[b]),
        Box::new(|r,a,b| r[a] + b),
        Box::new(|r,a,b| r[a] * r[b]),
        Box::new(|r,a,b| r[a] * b),
        Box::new(|r,a,b| r[a] & r[b]),
        Box::new(|r,a,b| r[a] & b),
        Box::new(|r,a,b| r[a] | r[b]),
        Box::new(|r,a,b| r[a] | b),
        Box::new(|r,a,_| r[a]),
        Box::new(|_,a,_| a),
        Box::new(|r,a,b| if a > r[b] { 1 } else { 0 }),
        Box::new(|r,a,b| if r[a] > b { 1 } else { 0 }),
        Box::new(|r,a,b| if r[a] > r[b] { 1 } else { 0 }),
        Box::new(|r,a,b| if a == r[b] { 1 } else { 0 }),
        Box::new(|r,a,b| if r[a] == b { 1 } else { 0 }),
        Box::new(|r,a,b| if r[a] == r[b] { 1 } else { 0 })
    ];

    // Get all opcode <=> operation matches
    let mut tmp: HashMap<usize, Vec<HashSet<usize>>> = HashMap::new();
    let mut three_or_more = 0;
    for chunk in &samples.into_iter().chunks(3) {
        let sample = chunk.into_iter().collect::<Vec<String>>();
        let r0 = numbers!(sample[0] => usize);
        let op = numbers!(sample[1] => usize);
        let r1 = numbers!(sample[2] => usize);

        let mut matches = HashSet::new();
        for (n,f) in ops.iter().enumerate() {
            let mut ar = [0usize; 4];
            ar.copy_from_slice(&r0[0..4]);
            if r1[op[3]] == f(ar, op[1], op[2]) {
                matches.insert(n);
            }
        }
        if matches.len() >= 3 {
            three_or_more += 1
        }
        tmp.entry(op[0]).or_default().push(matches);
    }
    println!("Part 1: {}", three_or_more);

    // Mapping between opcode and index in 'ops' vector
    let mut op_map = HashMap::new();
    let mut nf = 0;
    while nf < 16 {
        let mut to_remove = vec![]; // Remove from tmp when found
        for (&k, ref v) in tmp.iter() {
            let mut ii = v[0].clone();
            // Filter out previously mapped ops
            ii.retain(|e| !op_map.values().into_iter().any(|v| v == e));
            for ref b in v[1..].iter() {
                ii = ii.intersection(&b).map(|i| *i).collect();
                if ii.len() == 1 {
                    let idx = ii.into_iter().collect::<Vec<usize>>()[0];
                    to_remove.push(k);
                    op_map.insert(k, idx);
                    break
                }
            }
        }
        tmp.retain(|k,_| !to_remove.contains(k));
        nf = op_map.len();
    }
    // Finally, run program
    let mut ar = [0usize; 4];
    for p in program.trim().lines() {
        let op = numbers!(p => usize);
        let f = &ops[op_map[&op[0]]];
        ar[op[3]] = f(ar, op[1], op[2]);
    }
    println!("Part 2: {}", ar[0]);
}
