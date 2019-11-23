// Keywords: pathwalking

use std::collections::HashMap;

struct Cursor {
    pos: (i32, i32),
    chr: char,
    dir: (i32, i32),
}

fn next(input: &HashMap<(i32, i32), char>, cursor: &mut Cursor) -> bool {
    let (x, y) = cursor.pos;
    let (dx, dy) = cursor.dir;
    let mut directions = vec![];
    if cursor.chr == '+' {
        if dx == 0 {
            directions.push(( 1, 0));
            directions.push((-1, 0));
        }
        else if dy == 0 {
            directions.push((0,  1));
            directions.push((0, -1));
        }
    }
    else {
        directions.push(cursor.dir);
    }
    for dir in directions {
        let (dx, dy) = (dir.0 as i32, dir.1 as i32);
        if let Some(&chr) = input.get(&(x+dx, y+dy)) {
            cursor.pos = (x+dx, y+dy);
            cursor.dir = dir;
            cursor.chr = chr;
            return true;
        }
    }
    false
}

fn solve(input: &HashMap<(i32, i32), char>) -> (String, usize) {
    let mut string = String::new();
    let mut cursor = Cursor { pos: (75, 0), chr: '|', dir: (0, 1) };
    let mut steps = 1;
    while next(&input, &mut cursor) {
        if cursor.chr.is_alphabetic() {
            string.push(cursor.chr);
        }
        steps += 1;
    }
    (string, steps)
}

fn main() {
    let input: HashMap<(i32, i32), char> = include_str!("../../res/day19")
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line
                  .chars()
                  .enumerate()
                  .filter_map(|(x, c)| {
                      if c != ' ' {
                          return Some(((x as i32, y as i32), c));
                      }
                      None
                  })
                  .collect::<Vec<((i32, i32), char)>>())
        .collect();
    let (letters, steps) = solve(&input);
    println!("Part 1: {}", letters);
    println!("Part 2: {}", steps);
}
