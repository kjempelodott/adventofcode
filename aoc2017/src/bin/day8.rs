// Keywords: instructions, max

extern crate whiteread;
use std::collections::HashMap;
use whiteread::parse_string;

fn solve(input: &str) -> (i32, i32) {
    let mut max = 0;
    let mut regs: HashMap<String, i32> = HashMap::new();
    for line in input.lines() {
        let expr_test = line.split(" if ").collect::<Vec<&str>>();
        let (reg, sign, val) = parse_string::<(String, String, i32)>
            (expr_test[0]).unwrap();
        let (ifreg, op, ifval) = parse_string::<(String, String, i32)>
            (expr_test[1]).unwrap();
        let entry = regs.entry(ifreg).or_insert(0).clone();
        if match op.as_str() {
            ">" => entry > ifval,
            "<" => entry < ifval,
            ">=" => entry >= ifval,
            "<=" => entry <= ifval,
            "==" => entry == ifval,
            "!=" => entry != ifval,
            _ => unreachable!()
        } {
            let mut reg = regs.entry(reg).or_insert(0);
            *reg += val * if sign == "inc" { 1 } else { -1 };
            if *reg > max {
                max = *reg;
            }
        }
    }
    (*regs.values().max().unwrap(), max)
}

fn main() {
    let (highest, max) = solve(include_str!("../../res/day8"));
    println!("Part 1: {:?}", highest);
    println!("Part 2: {:?}", max);
}
