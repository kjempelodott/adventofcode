use std::collections::VecDeque;

#[derive(Clone)]
pub struct Program {
    heap: Vec<isize>,
    ip: usize,
    stdin: VecDeque<isize>,
    stdout: VecDeque<isize>,
    state: State
}

#[derive(Clone,Copy,PartialEq)]
pub enum State {
    Halted,
    Blocking,
    Running
}

use self::State::*;
impl Program {
    pub fn new(bytecode: Vec<isize>) -> Self {
        Program {
            heap: bytecode,
            ip: 0,
            stdin: VecDeque::new(),
            stdout: VecDeque::new(),
            state: Running
        }
    }


    pub fn get_state(&self) -> State {
        self.state
    }

    pub fn run_once(&self, stdin: Vec<isize>) -> isize {
        self.clone().run(stdin)
    }

    pub fn run(&mut self, _stdin: Vec<isize>) -> isize {
        let ref mut heap = self.heap;
        let ref mut stdin = self.stdin;
        let ref mut stdout = self.stdout;
        stdin.append(&mut _stdin.into_iter().collect());

        let mut i = self.ip;
        loop {
            let mut z = heap[i];
            let op = z % 100;
            let im2 = (z as f64).log10() >= 4.;
            if im2 { z -= 10000; }
            let im1 = (z as f64).log10() >= 3.;
            if im1 { z -= 1000; }
            let im0 = (z as f64).log10() >= 2.;

            match op {
                1 => {
                    let v1 = if im0 { heap[i+1] } else { heap[heap[i+1] as usize] };
                    let v2 = if im1 { heap[i+2] } else { heap[heap[i+2] as usize] };
                    let v3 = heap[i+3];
                    heap[v3 as usize] = v1 + v2;
                    i += 4;
                },
                2 => {
                    let v1 = if im0 { heap[i+1] } else { heap[heap[i+1] as usize] };
                    let v2 = if im1 { heap[i+2] } else { heap[heap[i+2] as usize] };
                    let v3 = heap[i+3];
                    heap[v3 as usize] = v1 * v2;
                    i += 4;
                },
                3 => {
                    let v1 = heap[i+1];
                    if let Some(val) = stdin.pop_front() {
                        heap[v1 as usize] = val;
                        i += 2;
                    }
                    else {
                        self.ip = i;
                        self.state = Blocking;
                        return *stdout.back().unwrap()
                    }
                },
                4 => {
                    let v1 = if im0 { heap[i+1] } else { heap[heap[i+1] as usize] };
                    stdout.push_back(v1);
                    i += 2;
                },
                5 => {
                    let v1 = if im0 { heap[i+1] } else { heap[heap[i+1] as usize] };
                    i = if v1 != 0 {
                        (if im1 { heap[i+2] } else { heap[heap[i+2] as usize] }) as usize
                    } else {
                        i + 3
                    }
                },
                6 => {
                    let v1 = if im0 { heap[i+1] } else { heap[heap[i+1] as usize] };
                    i = if v1 == 0 {
                        (if im1 { heap[i+2] } else { heap[heap[i+2] as usize] }) as usize
                    } else {
                        i + 3
                    }
                },
                7 => {
                    let v1 = if im0 { heap[i+1] } else { heap[heap[i+1] as usize] };
                    let v2 = if im1 { heap[i+2] } else { heap[heap[i+2] as usize] };
                    let v3 = heap[i+3];
                    heap[v3 as usize] = if v1 < v2 { 1 } else { 0 };
                    i += 4;
                },
                8 => {
                    let v1 = if im0 { heap[i+1] } else { heap[heap[i+1] as usize] };
                    let v2 = if im1 { heap[i+2] } else { heap[heap[i+2] as usize] };
                    let v3 = heap[i+3];
                    heap[v3 as usize] = if v1 == v2 { 1 } else { 0 };
                    i += 4;
                },
                _ => break
            }
        }
        self.ip = 0;
        self.state = Halted;
        *stdout.back().unwrap()
    }
}

