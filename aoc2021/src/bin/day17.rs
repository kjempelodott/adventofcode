#[macro_use]
extern crate aoc2021;
use aoc2021::read_from_stdin;

fn main() {
    let n: Vec<i64> = numbers!(read_from_stdin() => i64);
    let tx = (n[0] as u64, n[1] as u64);
    let ty = (n[2], n[3]);
    let mut max = 0;
    let mut count = 0;
    for _dx in 0u64..tx.1+1 {
        for _dy in ty.0..200 {
            let (mut dx, mut dy) = (_dx, _dy);
            let (mut x, mut y, mut y_max) = (0, 0, 0);
            for _ in 0.. {
                if y + dy < ty.0 {
                    if y < ty.1 {
                        max = std::cmp::max(max, y_max);
                    }
                    break
                }
                if x + dx > tx.1 {
                    break
                }
                x += dx;
                y += dy;
                y_max = std::cmp::max(y_max, y);
                dx = dx.saturating_sub(1);
                dy -= 1;
            }
            if y >= ty.0 && y <= ty.1 && x >= tx.0 && x <= tx.1 {
                count += 1;
            }
        }
    }
    println!("Part 1: {}", max);
    println!("Part 2: {}", count);
}
