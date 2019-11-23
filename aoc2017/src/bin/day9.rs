// Keywords: character stream, groups, compression, non-regex

fn solve(input: &str) -> (usize, usize) {
    let mut iter = input.chars();
    let mut groups = 0;
    let mut garbage = 0;
    let mut is_garbage = false;
    let mut val = 0;
    while let Some(c) = iter.next() {
        match c {
            '!' => { iter.next(); },
            '>' => is_garbage = false,
            _ if is_garbage => garbage += 1,
            '<' => is_garbage = true,
            '{' => val += 1,
            '}' => { groups += val; val -= 1 },
            _ => {}
        };
    }
    (garbage, groups)
}

fn main() {
    let (garbage, groups) = solve(include_str!("../../res/day9"));
    println!("Part 1: {}", groups);
    println!("Part 2: {}", garbage);
}
