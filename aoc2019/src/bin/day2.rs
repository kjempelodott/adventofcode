#[macro_use]
extern crate adventofcode2019;
use adventofcode2019::read_from_stdin;

fn main() {
    let ip: Vec<usize> = numbers!(read_from_stdin() => usize);
    println!("Part 1: {}", run(ip.clone(), 12, 2));
    for noun in 0..100 {
        for verb in 0..100 {
            if run(ip.clone(), noun, verb) == 19690720 {
                println!("Part 2: {}", 100 * noun + verb);
                break;
            }
        }
    }
}

fn run(mut ip: Vec<usize>, noun: usize, verb: usize) -> usize {
    ip[1] = noun;
    ip[2] = verb;
    let (mut i, mut op) = (0, ip[0]);
    while op != 99 {
        let (v1, v2, out) = (ip[i+1], ip[i+2], ip[i+3]);
        ip[out] = if op == 1 { ip[v1] + ip[v2] } else { ip[v1] * ip[v2] };
        i += 4;
        op = ip[i];
    }
    return ip[0];
}

    
