use std::collections::VecDeque;

#[derive(Clone)]
pub struct Program(Vec<isize>);

impl Program {
    pub fn new(bytecode: Vec<isize>) -> Self {
        Program(bytecode)
    }

    pub fn run_once(&self, stdin: Vec<isize>) -> isize {
        self.clone().run(stdin)
    }

    pub fn run(&mut self, stdin: Vec<isize>) -> isize {
        let ref mut heap = self.0;
        let mut i = 0;
        let mut stdin = VecDeque::from(stdin);
        let mut stdout = VecDeque::new();

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
                    heap[v1 as usize] = stdin.pop_front().unwrap();
                    i += 2;
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
        *stdout.back().unwrap()
    }
}

