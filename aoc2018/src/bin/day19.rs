extern crate adventofcode2018;
use adventofcode2018::read_from_stdin;

fn run(instr: &Vec<(&str, Vec<usize>)>, reg: &mut [usize; 6], ip: usize) {
    while reg[ip] < instr.len() - 1 {
        let (ii, ref v) = instr[reg[ip]];
        match ii {
            "addr" => reg[v[2]] = reg[v[0]] + reg[v[1]],
            "addi" => reg[v[2]] = reg[v[0]] + v[1],
            "mulr" => reg[v[2]] = reg[v[0]] * reg[v[1]],
            "muli" => reg[v[2]] = reg[v[0]] * v[1],
            "banr" => reg[v[2]] = reg[v[0]] & reg[v[1]],
            "bani" => reg[v[2]] = reg[v[0]] & v[1],
            "borr" => reg[v[2]] = reg[v[0]] | reg[v[1]],
            "bori" => reg[v[2]] = reg[v[0]] | v[1],
            "setr" => reg[v[2]] = reg[v[0]],
            "seti" => reg[v[2]] = v[0],
            "gtir" => reg[v[2]] = if v[0] > reg[v[1]] { 1 } else { 0 },
            "gtri" => reg[v[2]] = if reg[v[0]] > v[1] { 1 } else { 0 },
            "gtrr" => reg[v[2]] = if reg[v[0]] > reg[v[1]] { 1 } else { 0 },
            "eqir" => reg[v[2]] = if v[0] == reg[v[1]] { 1 } else { 0 },
            "eqri" => reg[v[2]] = if reg[v[0]] == v[1] { 1 } else { 0 },
            "eqrr" => reg[v[2]] = if reg[v[0]] == reg[v[1]] { 1 } else { 0 },
            _ => unreachable!()
        }
        reg[ip] += 1;
    }

}

fn main() {
    let input = read_from_stdin();
    let mut instr: Vec<(&str, Vec<usize>)> = input.lines()
        .map(|l| {
            let parts = l.split_whitespace().collect::<Vec<&str>>();
            let num = parts.iter().filter_map(|n| n.parse().ok()).collect();
            (parts[0], num)
        })
        .collect();

    let mut reg = [0usize; 6];
    let ip = instr.remove(0).1[0];
    run(&instr, &mut reg, ip);
    println!("Part 1: {}", reg[0]);

    // Cycle unitil reg[4] < reg[5]
    // -----------------------------
    // 0 1 0      3 10551347 337584
    // 0 1 337584 4 10551347 337584
    // 0 1 0      5 10551347 337584
    // 0 1 0      6 10551347 337584
    // 0 1 0      8 10551347 337584
    // 0 1 0      9 10551347 337585
    // 0 1 0     10 10551347 337585
    // 0 1 0     11 10551347 337585
    // Cycle unitil reg[4] == reg[2]
    // -----------------------------
    // 1 2 0      3 10551347 337584
    // 1 2 675168 4 10551347 337584
    // 1 2 0      5 10551347 337584
    // 1 2 0      6 10551347 337584
    // 1 2 0      8 10551347 337584
    // 1 2 0      9 10551347 337585
    // 1 2 0     10 10551347 337585
    // 1 2 0     11 10551347 337585

    // 10551347 = 73 * 144539
    // 0,      1,        0, 5, 10551347, 10551346
    // 1,      2,        0, 5, 10551347, 10551346
    // 1,      72,       0, 5, 10551347, 10551346
    // 74,     144539,   0, 5, 10551347, 72
    // 144613, 10551346, 0, 5, 10551347, 10551346

    println!("Part 2: {}", 1 + 73 + 144539 + 10551347);
}
