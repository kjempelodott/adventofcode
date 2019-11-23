// KEywords: hex grid, distance

fn mov(dir: &str) -> (i32, i32) {
    match dir {
        "n"  => ( 2, 0),
        "s"  => (-2, 0),
        "nw" => ( 1,-1),
        "sw" => (-1,-1),
        "ne" => ( 1, 1),
        "se" => (-1, 1),
        _ => unreachable!()
    }
}

fn solve(input: &str) -> (i32, i32) {
    let distance = |y: i32, x: i32| (y.abs() + x.abs())/2 + 0.max(x.abs() - y.abs());
    let mut maxd = 0;
    let (y, x) = input.split(',')
        .map(|s| mov(s))
        .fold((0, 0), |s, dydx| {
            let (mut y, mut x) = s;
            let (dy, dx) = dydx;
            y += dy;
            x += dx;
            let dist = distance(y, x);
            if dist > maxd {
                maxd = dist;
            }
            (y, x)
        });
    (distance(y, x), maxd)
}

fn main() {
    let (s, m) = solve(include_str!("../../res/day11").trim());
    println!("Part 1: {}", s);
    println!("Part 2: {}", m);
}
