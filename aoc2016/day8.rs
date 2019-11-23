use std::thread::sleep;
use std::time::Duration;

fn display(screen: &Vec<Vec<char>>, clear: bool) -> () {
    for row in screen.iter() {
        println!("{}", row.iter().map(|&c| c).collect::<String>());
    }
    sleep(Duration::from_millis(50));
    if clear { println!("\x1b[7A") }
}

fn main() {
    let mut screen = vec![vec![' ';50];6];
    for instr in include_str!("input8").trim().lines() {
        let mut z = instr.split_whitespace();
        match z.next() {
            Some("rect") => {
                let mut w = z.next().unwrap().split("x").map(|w| w.parse().unwrap());
                let (y, x) = (w.next().unwrap(), w.next().unwrap());
                for i in 0..y  {
                    for j in 0..x {
                        screen[j][i] = '\u{2588}';
                    }
                }
            },
            Some("rotate") => {
                let kind = z.next();
                let j: usize = z.next().unwrap().split("=").nth(1).unwrap().parse().unwrap();
                let n: usize = z.nth(1).unwrap().parse().unwrap();
                match kind {
                    Some("row") => {
                        let rot: Vec<char> = (50-n..50).chain(0..50-n)
                            .map(|i| screen[j][i]).collect();
                        screen[j] = rot;
                    },
                    Some("column") => {
                        let rot: Vec<char> = (6-n..6).chain(0..6-n)
                            .map(|i| screen[i][j]).collect();
                        for i in 0..6 {
                            screen[i][j] = rot[i];
                        }
                    },
                    _ => {},
                }},
            _ => {}
        }
        display(&screen, true);
    }
    display(&screen, false);
    println!("{}", screen.into_iter()
             .fold(0, |s, x|  s + x.into_iter().filter(|&y| y != ' ').count()));
}
