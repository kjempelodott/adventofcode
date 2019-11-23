use std::collections::HashSet;

fn run(last: &mut usize) -> usize {
    let mut r0 = HashSet::new();
    let mut r1 = 0usize;
    let mut r3;
    'outer : loop {
        r3 = r1 | 65536;
        r1 = 10905776;
        'inner : loop {
            r1 = (((r1 + (r3 & 255)) & 16777215) * 65899) & 16777215;
            if 256 > r3 {
                if *last == 0 {
                    return r1
                }
                if r0.insert(r1) {
                    *last = r1;
                    continue 'outer;
                }
                return *last;
            }
            r3 /= 256;
        }
    }
}

fn main() {
    let mut low = run(&mut 0);
    println!("Part 1: {}", low);
    println!("Part 2: {}", run(&mut low));
}
