fn main() {
    let x: Vec<Vec<u16>> = include_str!("input3").trim().lines()
        .map(|l| l.split_whitespace()
             .map(|x| x.parse().unwrap())
             .collect())
        .collect();
    // Part 1
    println!("{}", x.clone().into_iter().fold(0, |c, mut y: Vec<u16>| {
        y.sort();
        c + (y[2] < y[0] + y[1]) as u16
    }));

    // Part 2
    println!("{}", x.chunks(3)
             .map(|w| {
                 (0..3).filter(|&i| {
                     let mut y = vec!(w[0][i], w[1][i], w[2][i]);
                     y.sort();
                     y[2] < y[0] + y[1]
                 }).count()
             }).sum::<usize>());
}
