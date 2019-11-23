use std::collections::HashSet;

fn main() {
    let b = include_str!("input1");
    let mut s: [i32; 2] = [0, 0];
    let mut d: [i32; 2] = [1, 1];
    let mut xy: usize = 0;
    let mut p = "";
    let mut loc = HashSet::new();
    let mut t = false;
    
    for i in b.split(", ") {
        let (a, b) = i.split_at(1);
        let l: i32 = b.parse().unwrap();
        if p == a { d[xy] *= -1; }
        if t { s[xy] += l*d[xy]; }
        else {
            for z in 0..l {
                s[xy] += d[xy];
                if loc.contains(&s) {
                    t = true;
                    println!("Blocks to first location visited twice: {}",
                             s[0].abs() + s[1].abs());
                    s[xy] += (l - z - 1)*d[xy];
                    break;
                }
                loc.insert(s);
            }
        }
        xy = 1 - xy;
        p = a;
    }
    println!("Blocks to Easter Bunny HQ: {}", s[0].abs() + s[1].abs());
}
