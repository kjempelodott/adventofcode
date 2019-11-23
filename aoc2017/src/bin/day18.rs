// Keywords: instructions, ipc, jumps

#[macro_use]
extern crate lazy_static;
extern crate whiteread;
use std::collections::{VecDeque, BTreeMap};
use whiteread::parse_string;

struct Sound {
    op: String,
    arg: String
}

struct Ipc {
    op: String,
    arg: String
}

struct Op {
    op: String,
    arg1: String,
    arg2: String
}

struct Cpu {
    regs: BTreeMap<String, i64>,
    ipc: VecDeque<i64>,
    sp: usize,
    freq: i64,
    halt: bool
}    

impl Cpu {
    fn new(idx: i64) -> Self {
        Cpu { regs: (97u8..123)
              .map(|c| ([c as char].iter().collect(), idx))
              .collect(),
              ipc:  VecDeque::new(),
              sp:   0,
              freq: 0,
              halt: false
        }
    }

    fn into_value(&self, v: &String) -> i64 {
        if let Some(v) = self.regs.get(v) {
            return *v;
        }
        v.parse().unwrap()
    }

    fn next(&mut self, p: &Vec<Box<Instruction>>, other: Option<&mut Cpu>) {
        p[self.sp].exec(self, other);
    }
}

trait Instruction where Self: Sync {
    fn try_new(instr: &str) -> Option<Self> where Self: Sized;
    fn exec(&self, cpu: &mut Cpu, other: Option<&mut Cpu>);
}

impl Instruction for Sound {
    fn try_new(instr: &str) -> Option<Self> {
        if let Ok((op, arg)) = parse_string::<(String, String)>(instr) {
            return Some(Sound { op: op, arg: arg })
        }
        None
    }
    fn exec(&self, cpu: &mut Cpu, _: Option<&mut Cpu>) {
        let val = cpu.into_value(&self.arg);
        match self.op.as_str() {
            "snd" => cpu.freq = val,
            "rcv" => if val != 0 {
                cpu.halt = true;
                return;
            },
            _ => unreachable!()
        }
        cpu.sp += 1;
    }
}

impl Instruction for Ipc {
    fn try_new(instr: &str) -> Option<Self> {
        if let Ok((op, arg)) = parse_string::<(String, String)>(instr) {
            return Some(Ipc { op: op, arg: arg })
        }
        None
    }
    fn exec(&self, cpu: &mut Cpu, other: Option<&mut Cpu>) {
        match self.op.as_str() {
            "snd" => {
                other.unwrap().ipc.push_back(cpu.into_value(&self.arg));
                cpu.freq += 1;
            }
            "rcv" => {
                if let Some(val) = cpu.ipc.pop_front() {
                    *cpu.regs.get_mut(&self.arg).unwrap() = val;
                }
                else {
                    cpu.halt = true;
                    return;
                }
            },
            _ => unreachable!()
        }
        cpu.sp += 1;
    }
}

impl Instruction for Op {
    fn try_new(instr: &str) -> Option<Self> {
        if let Ok((op, arg1, arg2)) =
            parse_string::<(String, String, String)>(instr) {
                return Some(Op { op: op, arg1: arg1, arg2: arg2 })
            }
        None
    }
    fn exec(&self, cpu: &mut Cpu, _: Option<&mut Cpu>) {
        let val = cpu.into_value(&self.arg2);
        if let Some(reg) = cpu.regs.get_mut(&self.arg1) {
            match self.op.as_str() {
                "set" => *reg = val,
                "add" => *reg += val,
                "mul" => *reg *= val,
                "mod" => *reg %= val,
                "jgz" => if *reg > 0 {
                    cpu.sp = (cpu.sp as i64 + val) as usize;
                    return
                },
                _ => unreachable!()
            }
        }
        else {
            // jgz
            if self.arg1.parse::<i64>().unwrap() > 0 {
                cpu.sp = (cpu.sp as i64 + val) as usize;
                return
            }
        }
        cpu.sp += 1;
    }
}

lazy_static!{
    static ref PROGRAM0: Vec<Box<Instruction>> = include_str!("../../res/day18")
        .lines()
        .map(|line| {
            if let Some(sound) = Sound::try_new(line) {
                return Box::new(sound) as Box<Instruction>;
            }
            if let Some(op) = Op::try_new(line) {
                return Box::new(op) as Box<Instruction>;
            }
            unreachable!()
        })
        .collect();

    static ref PROGRAM1: Vec<Box<Instruction>> = include_str!("../../res/day18")
        .lines()
        .map(|line| {
            if let Some(ipc) = Ipc::try_new(line) {
                return Box::new(ipc) as Box<Instruction>;
            }
            if let Some(op) = Op::try_new(line) {
                return Box::new(op) as Box<Instruction>;
            }
            unreachable!()
        })
        .collect();
}

fn solve_part1() -> i64 {
    let mut cpu = Cpu::new(0);
    while !cpu.halt {
        cpu.next(&PROGRAM0, None);
    }
    cpu.freq
}

fn solve_part2() -> i64 {
    let mut cpu0 = Cpu::new(0);
    let mut cpu1 = Cpu::new(1);
    while !cpu0.halt || !cpu1.halt {
        cpu0.next(&PROGRAM1, Some(&mut cpu1));
        cpu1.next(&PROGRAM1, Some(&mut cpu0));
    }
    cpu1.freq
}

fn main() {
    println!("Part 1: {}", solve_part1());
    println!("Part 2: {}", solve_part2());
}
