#[macro_use]
extern crate adventofcode2019;
use adventofcode2019::read_from_stdin;

use std::collections::VecDeque;

fn main() {
    let vm = numbers!(read_from_stdin() => isize);

    println!("Part 1: {}", solve(vm.clone(), 1));
    println!("Part 2: {}", solve(vm.clone(), 5));
}

fn solve(mut vm: Vec<isize>, id: isize) -> isize {
    let mut i = 0;
    let mut stdout = VecDeque::new();
    stdout.push_back(id);

    loop {
        let mut z = vm[i];
        let op = z % 100;
        let im2 = (z as f64).log10() >= 4.;
        if im2 { z -= 10000; }
        let im1 = (z as f64).log10() >= 3.;
        if im1 { z -= 1000; }
        let im0 = (z as f64).log10() >= 2.;

        match op {
            1 => {
                let v1 = if im0 { vm[i+1] } else { vm[vm[i+1] as usize] };
                let v2 = if im1 { vm[i+2] } else { vm[vm[i+2] as usize] };
                let v3 = vm[i+3];
                vm[v3 as usize] = v1 + v2;
                i += 4;
            },
            2 => {
                let v1 = if im0 { vm[i+1] } else { vm[vm[i+1] as usize] };
                let v2 = if im1 { vm[i+2] } else { vm[vm[i+2] as usize] };
                let v3 = vm[i+3];
                vm[v3 as usize] = v1 * v2;
                i += 4;
            },
            3 => {
                let v1 = vm[i+1];
                vm[v1 as usize] = stdout.pop_front().unwrap();
                i += 2;
            },
            4 => {
                let v1 = if im0 { vm[i+1] } else { vm[vm[i+1] as usize] };
                stdout.push_back(v1);
                i += 2;
            },
            5 => {
                let v1 = if im0 { vm[i+1] } else { vm[vm[i+1] as usize] };
                i = if v1 != 0 {
                    (if im1 { vm[i+2] } else { vm[vm[i+2] as usize] }) as usize
                } else {
                    i + 3
                }
            },
            6 => {
                let v1 = if im0 { vm[i+1] } else { vm[vm[i+1] as usize] };
                i = if v1 == 0 {
                    (if im1 { vm[i+2] } else { vm[vm[i+2] as usize] }) as usize
                } else {
                    i + 3
                }
            },
            7 => {
                let v1 = if im0 { vm[i+1] } else { vm[vm[i+1] as usize] };
                let v2 = if im1 { vm[i+2] } else { vm[vm[i+2] as usize] };
                let v3 = vm[i+3];
                vm[v3 as usize] = if v1 < v2 { 1 } else { 0 };
                i += 4;
            },
            8 => {
                let v1 = if im0 { vm[i+1] } else { vm[vm[i+1] as usize] };
                let v2 = if im1 { vm[i+2] } else { vm[vm[i+2] as usize] };
                let v3 = vm[i+3];
                vm[v3 as usize] = if v1 == v2 { 1 } else { 0 };
                i += 4;
            },
            _ => break
        }
    }
    *stdout.back().unwrap()
}
