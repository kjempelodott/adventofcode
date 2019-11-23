extern crate adventofcode2018;
extern crate min_max_heap;
use adventofcode2018::{into_lines,read_from_stdin};
use min_max_heap::MinMaxHeap;
use std::collections::HashMap;

struct Worker {
    task: Option<char>,
    eta: u8
}

impl Worker {
    fn new() -> Self {
        Worker {
            task: None,
            eta: 0
        }
    }
}

// Scheduling
fn do_work(n: usize, instr: &Vec<String>) -> (String, usize) {
    let mut workers: Vec<Worker> = (0..n).map(|_| Worker::new()).collect();
    let mut blocked_by: HashMap<char, Vec<char>> = HashMap::new();
    for line in instr.iter() {
        let mut c = line.chars();
        let (a, b) = (c.nth(5).unwrap(), c.nth(30).unwrap());
        blocked_by.entry(b).or_default().push(a);
    }

    let mut time_spent = 0;
    let mut done = String::new();
    let mut q = MinMaxHeap::new(); // Available tasks
    q.push(instr[0].chars().nth(5).unwrap());
    loop {
        let (mut idle, mut busy): (Vec<&mut Worker>, Vec<&mut Worker>) = workers
            .iter_mut()
            .partition(|w| w.task.is_none());  
        // Fast forward in time
        let dt = busy.iter().map(|ref w| w.eta).min().unwrap_or(0);
        for w in busy.iter_mut() {
            w.eta -= dt;
            if w.eta == 0 {
                done.push(w.task.take().unwrap());
                idle.push(w);
            }
        }
        time_spent += dt as usize;
        // Any more tasks?
        if blocked_by.len() == 0 {
            break
        }
        // Unlock tasks
        let tasks = blocked_by.keys().cloned()
            .filter(|&t| blocked_by[&t].iter().all(|&x| done.contains(x)))
            .collect::<Vec<char>>();
        tasks.iter().for_each(|t| q.push(blocked_by.remove_entry(&t).unwrap().0));
        // Assign tasks to idle workers
        idle.iter_mut().for_each(|w| {
            if let Some(task) = q.pop_min() {
                w.task = Some(task);
                w.eta = (task as u8) - 4;
            }
        });
    }
    (done, time_spent)
}

fn main() {
    let instr = into_lines(read_from_stdin());
    println!("Part 1: {}", do_work(1, &instr).0);
    println!("Part 2: {}", do_work(5, &instr).1);
}
