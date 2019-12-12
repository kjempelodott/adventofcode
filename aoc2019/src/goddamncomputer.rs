use std::collections::VecDeque;

#[derive(Clone,Copy,PartialEq)]
pub enum State {
    Halted,
    Blocking,
    Running
}

#[derive(Clone,Copy,PartialEq,Debug)]
enum Mode {
    Positional,
    Immediate,
    Relative
}

use self::State::*;
use self::Mode::*;

#[derive(Clone)]
struct Memory(Vec<isize>);

#[derive(Clone)]
pub struct Program {
    baseaddr: isize,
    heap: Memory,
    ip: usize,
    stdin: VecDeque<isize>,
    stdout: VecDeque<isize>,
    state: State
}

struct OpCode;
impl OpCode {
    fn resolve(mut z: isize) -> (isize, Mode, Mode, Mode) {
        let op = z % 100;
        let mode2 = match z {
            _ if z > 20000 => { z -= 20000; Relative },
            _ if z > 10000 => { z -= 10000; Immediate },
            _ => Positional
        };
        let mode1 = match z {
            _ if z > 2000 => { z -= 2000; Relative },
            _ if z > 1000 => { z -= 1000; Immediate },
            _ => Positional
        };
        let mode0 = match z {
            _ if z > 200 => Relative,
            _ if z > 100 => Immediate,
            _ => Positional
        };
        (op, mode0, mode1, mode2)
    }
}

impl Memory {
    fn w(&mut self, i: usize, val: isize) {
        if i < self.0.len() {
            self.0[i] = val;
        }
        else {
            self.0.resize(i, 0);
            self.0.push(val);
        }
    }
    fn r(&self, i: usize) -> isize {
        if i >= self.0.len() { 0 } else { self.0[i] }
    }
}

impl Program {
    pub fn new(intcode: Vec<isize>) -> Self {
        Program {
            baseaddr: 0,
            heap: Memory(intcode),
            ip: 0,
            stdin: VecDeque::new(),
            stdout: VecDeque::new(),
            state: Running
        }
    }

    fn get_val(&self, i: usize, m: Mode) -> isize {
        match m {
            Relative => self.heap.r((self.heap.r(i) + self.baseaddr) as usize),
            Immediate => self.heap.r(i),
            Positional => self.heap.r(self.heap.r(i) as usize)
        }
    }

    fn set_val(&mut self, i: usize, m: Mode, val: isize) {
        match m {
            Relative => {
                let pos = self.heap.r(i) + self.baseaddr;
                self.heap.w(pos as usize, val);
            },
            Immediate => panic!(),
            Positional => {
                let pos = self.heap.r(i);
                self.heap.w(pos as usize, val);
            }
        }
    }

    pub fn get_state(&self) -> State {
        self.state
    }

    pub fn run_once(&self, stdin: Vec<isize>) -> Vec<isize> {
        self.clone().run(stdin)
    }

    pub fn run(&mut self, _stdin: Vec<isize>) -> Vec<isize> {
        self.stdin.append(&mut _stdin.into_iter().collect());
        let mut i = self.ip;
        loop {
            let (op, m0, m1, m2) = OpCode::resolve(self.heap.r(i));
            match op {
                1 => {
                    let v1 = self.get_val(i+1, m0);
                    let v2 = self.get_val(i+2, m1);
                    self.set_val(i+3, m2, v1+v2);
                    i += 4;
                },
                2 => {
                    let v1 = self.get_val(i+1, m0);
                    let v2 = self.get_val(i+2, m1);
                    self.set_val(i+3, m2, v1*v2);
                    i += 4;
                },
                3 => {
                    if let Some(val) = self.stdin.pop_front() {
                        self.set_val(i+1, m0, val);
                        i += 2;
                    }
                    else {
                        self.ip = i;
                        self.state = Blocking;
                        return self.stdout.drain(..).collect()
                    }
                },
                4 => {
                    let v1 = self.get_val(i+1, m0);
                    self.stdout.push_back(v1);
                    i += 2;
                },
                5 => {
                    let v1 = self.get_val(i+1, m0);
                    i = if v1 != 0 {
                        self.get_val(i+2, m1) as usize
                    } else {
                        i + 3
                    }
                },
                6 => {
                    let v1 = self.get_val(i+1, m0);
                    i = if v1 == 0 {
                        self.get_val(i+2, m1) as usize
                    } else {
                        i + 3
                    }
                },
                7 => {
                    let v1 = self.get_val(i+1, m0);
                    let v2 = self.get_val(i+2, m1);
                    self.set_val(i+3, m2, if v1 < v2 { 1 } else { 0 });
                    i += 4;
                },
                8 => {
                    let v1 = self.get_val(i+1, m0);
                    let v2 = self.get_val(i+2, m1);
                    self.set_val(i+3, m2, if v1 == v2 { 1 } else { 0 });
                    i += 4;
                },
                9 => {
                    self.baseaddr += self.get_val(i+1, m0);
                    i += 2;
                },
                _ => break
            }
        }
        self.ip = 0;
        self.state = Halted;
        self.stdout.drain(..).collect()
    }
}

