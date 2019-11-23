fn mov(d: &mut (i8, i8), c: u8, m: i8, p2: bool) -> Option<(usize, usize)> {
    let x = match c {
        b'U' => (-1,0),
        b'D' => (1, 0),
        b'L' => (0,-1),
        b'R' => (0, 1),
        _ => { return Some((d.0 as usize, d.1 as usize)); },
    };
    let t = (d.0 + x.0, d.1 + x.1);
    if t.0 < m && t.0 > -1 && t.1 < m && t.1 > -1 &&
        (!p2 || (t.0 - 2).abs() + (t.1 - 2).abs() < 3) { *d = t; }
    None
}

fn main() {
    let np = vec![vec!["1", "2", "3"],
                  vec!["4", "5", "6"],
                  vec!["7", "8", "9"]];
    
    let np2 = vec![vec!["", "",  "1", "", "" ],
                   vec!["", "2", "3", "4","" ],
                   vec!["5","6", "7", "8","9"],
                   vec!["", "A", "B", "C","" ],
                   vec!["", "",  "D", "", ""]];
    
    let b = include_bytes!("input2");
    let mut d: (i8, i8) = (1, 1);
    println!("{}", b.clone().into_iter()
             .filter_map(|x| mov(&mut d, *x, 3, false))
             .fold(String::new(), |s, x| s + np[x.0][x.1]));
    let mut d: (i8, i8) = (2, 2);
    println!("{}", b.into_iter()
             .filter_map(|x| mov(&mut d, *x, 5, true))
             .fold(String::new(), |s, x| s + np2[x.0][x.1]));
}

    
