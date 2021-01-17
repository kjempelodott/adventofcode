#[macro_use]
extern crate adventofcode2020;
use adventofcode2020::read_from_stdin;

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}
 
fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}
 
fn chinese_remainder(residues: &[i64], modulii: &[i64], lcm: i64) -> Option<i64> {
    let mut sum = 0; 
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = lcm / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }
    Some(sum % lcm)
}

fn main() {
    let input = read_from_stdin();
    let ids: Vec<isize> = numbers![input => isize];
    let ts = ids[0];
    let fbus = ids[1..].iter().min_by_key(|&&x| x - ts % x).unwrap();
    println!("Part 1: {}", fbus * (fbus - ts % fbus));

    let (offsets, bids): (Vec<_>, Vec<_>) = input.lines().nth(1).unwrap()
        .split(|c| c == ',')
        .enumerate()
        .filter_map(|(i,s)| {
            if let Ok(n) = s.parse::<i64>() {
                return Some((-(i as i64), n as i64))
            }
            None
        })
        .unzip();

    let lcm = bids.iter().product::<i64>();
    if let Some(mut sol) = chinese_remainder(&offsets[..], &bids[..], lcm) {
        while sol < -1 {
            sol += lcm;
        }
        println!("Part 2: {}", sol);
    }
}
