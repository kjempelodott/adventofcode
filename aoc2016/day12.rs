use std::collections::HashMap;

fn get_a(regs: &mut HashMap<&str, i32>) {
    let instr: Vec<&str> = include_str!("input12").trim().lines().collect();
    let mut i = 0;
    while i < instr.len() {
        let ins: Vec<&str> = instr[i].split_whitespace().collect();
        match ins[0] {
            "cpy" => { *regs.get_mut(ins[2]).unwrap() = *regs.get(ins[1])
                        .unwrap_or(&ins[1].parse().unwrap_or(0)) },
            "inc" => { *regs.get_mut(ins[1]).unwrap() += 1 },
            "dec" => { *regs.get_mut(ins[1]).unwrap() -= 1 },
            "jnz" => {
                if *regs.get(ins[1]).unwrap_or(&ins[1].parse().unwrap_or(0)) != 0 {
                    i += ins[2].parse::<i32>().unwrap() as usize - 1;
                }
            },
            _ => {} ,
        }
        i += 1;
    }
    println!("{}", regs.get("a").unwrap());
}

fn main() {
    // Part 1
    let mut regs = vec![("a",0),("b",0),("c",0),("d",0)]
        .into_iter().collect::<HashMap<&str, i32>>();
    get_a(&mut regs);
    // Part 2
    let mut regs = vec![("a",0),("b",0),("c",1),("d",0)]
        .into_iter().collect::<HashMap<&str, i32>>();
    get_a(&mut regs);
}
