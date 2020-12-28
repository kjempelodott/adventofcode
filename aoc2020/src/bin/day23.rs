extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;
use std::iter::successors;

fn play(cups: &[usize], moves: usize) -> Vec<usize> {
    // Use a linked list to keep track of order
    // C = [a, b, c, ...]
    // LL[C[a]] = C[b]
    // LL[C[b]] = C[c]
    
    let n = cups.len();
    let max = *cups.iter().max().unwrap();
    let min = *cups.iter().min().unwrap();
    let mut ll = vec![0; n+1];
    (0..n).for_each(|i| ll[cups[i]] = cups[(i+1) % n]);

    let mut c = cups[0];
    for _ in 0..moves {
        let mut three = successors(Some(ll[c]), |x| Some(ll[*x])).take(3).collect::<Vec<usize>>();
        // Link current cup to the cup after the third removed cup
        ll[c] = ll[three[2]];
        // Find destination
        let mut d = if c > min { c - 1} else { max };
        while three.contains(&d) {
            d = if d > min { d - 1} else { max };
        }
        // Insert the three cups at destination
        three.push(ll[d]);
        [d].iter().chain(&three).zip(&three).for_each(|(i,x)| ll[*i] = *x);
        // Go to next cup
        c = ll[c];
    }
    return ll;
}


fn main() {
    let mut cups = read_from_stdin().bytes().map(|x| x as usize - 48).collect::<Vec<_>>();
    let mut ll = play(&cups, 100);
    println!("Part 1: {}", successors(Some(ll[1]), |x| Some(ll[*x]))
             .take(8)
             .map(|x| (x as u8 + 48) as char)
             .collect::<String>());

    (10..1_000_001).for_each(|x| cups.push(x));
    ll = play(&cups, 10_000_000);
    println!("Part 2: {}", ll[1] * ll[ll[1]]);
}
