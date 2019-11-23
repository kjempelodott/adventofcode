use std::collections::VecDeque;

const N: usize = 3014387;

fn main() {
    // Part 1
    let mut elves: VecDeque<usize> = (1..N+1).collect();
    while elves.len() > 1 {
        let this = elves.pop_front().unwrap();
        elves.push_back(this);
        elves.pop_front().unwrap();
    }
    println!("Part 1: Elf number {} gets all the presents", elves.front().unwrap());

    // Part 2
    let mut half1: VecDeque<usize> = (1..(N+1)/2).collect();
    let mut half2: VecDeque<usize> = ((N+1)/2..N+1).collect();
    while half1.len() > 0 {
        half2.pop_front();
        half2.push_back(half1.pop_front().unwrap());
        if (half1.len() + half2.len()) % 2  == 0 {
            half1.push_back(half2.pop_front().unwrap());
        }
    }
    println!("Part 2: Elf number {} gets all the presents", half2.front().unwrap());
}
