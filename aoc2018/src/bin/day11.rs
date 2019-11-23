const SERIAL_NO: isize = 1133;

fn val(x: isize, y: isize) -> isize {
    let rack_id = x + 10;
    let power = rack_id * (rack_id * y + SERIAL_NO);
    if power < 100 {
        return 0
    }
    return ((power / 100) % 10) - 5
}

fn max_square(grid: &Vec<Vec<isize>>, size: usize) -> (isize, (usize, usize)) {
    let (mut v, mut xy) = (0, (0, 0));
    for x in 0..301 - size {
        for y in 0..301 - size {
            let power = (x..x+size)
                .map(|x| (y..y+size).map(|y| grid[x][y]).sum::<isize>())
                .sum::<isize>();
            if power > v {
                v = power;
                xy = (x, y);
            }
        }
    }
    (v, xy)
}

fn main() {
    let grid: Vec<Vec<isize>> = (0..300)
        .map(|x| (0..300).map(|y| val(x, y)).collect())
        .collect();
    let part1 = max_square(&grid, 3);
    println!("Part 1: {},{}", (part1.1).0, (part1.1).1);
    let part2 = (3..300).map(|s| (s, max_square(&grid, s)))
        .max_by_key(|(_, ms)| ms.0)
        .unwrap();
    println!("Part 2: {},{},{}", ((part2.1).1).0, ((part2.1).1).1, part2.0);
}
