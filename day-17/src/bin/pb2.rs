use std::{io::Write, ops::Shr};

use rayon::iter::{ParallelBridge, ParallelIterator};

type Tryte = u8;

#[derive(Debug)]
struct Computer<'p> {
    program: &'p Vec<Tryte>,
    register_a: u128,
    register_b: u128,
    register_c: u128,
    ip: usize,
    stdout: Vec<u8>,
}

impl<'p> Computer<'p> {
    fn reset(&mut self) {
        self.ip = 0;
        self.stdout = Vec::new();
    }

    fn decompile_one(&self, ip: usize) -> String {
        if self.ip >= self.program.len() {
            return "hlt".to_string();
        }
        let instruction = self.program[ip];
        let operand = self.program[ip + 1];
        let combo = match operand {
            v @ 0x0..=0x3 => format!("{v}"),
            0x4 => "reg_a".to_string(),
            0x5 => "reg_b".to_string(),
            0x6 => "reg_c".to_string(),
            0x7 => "?".to_string(),
            _ => unreachable!(),
        };
        let string = match instruction {
            // adv
            0x0 => format!("adv {}", combo),
            // bxl
            0x1 => {
                format!("bxl {operand}")
            }
            // bst
            0x2 => {
                format!("bst {}", combo)
            }
            // jnz
            0x3 => format!("jnz {}", operand),
            // bxc
            0x4 => format!("bxc"),
            // out
            0x5 => format!("out {}", combo),
            // bdv
            0x6 => format!("bdv {}", combo),
            // cdv
            0x7 => format!("cdv {}", combo),
            _ => unreachable!(),
        };
        string
    }

    fn decompile(&self) -> Vec<String> {
        let mut res = Vec::new();
        let mut ip = 0;
        loop {
            if ip >= self.program.len() {
                break;
            }
            res.push(self.decompile_one(ip));

            ip += 2;
        }
        res
    }

    fn new(program: &'p Vec<Tryte>) -> Self {
        Self {
            program,
            register_a: 0,
            register_b: 0,
            register_c: 0,
            ip: 0,
            stdout: Vec::new(),
        }
    }

    pub fn run(&mut self, cb: impl Fn(&Self) -> Option<()>) {
        loop {
            if self.ip >= self.program.len() {
                break;
            }

            let instruction = self.program[self.ip];
            self.ip += 1;
            let operand = self.program[self.ip];
            self.ip += 1;

            match instruction {
                // adv
                0x0 => {
                    let numerator = self.register_a;
                    let actual_operand = self.combo(operand);
                    let divisor = 1 << actual_operand;
                    self.register_a = numerator / divisor;
                }
                // bxl
                0x1 => {
                    let left = self.register_b;
                    let right = operand as u128;
                    self.register_b = left ^ right;
                }
                // bst
                0x2 => {
                    let actual_operand = self.combo(operand);
                    self.register_b = actual_operand % 8;
                }
                // jnz
                0x3 => {
                    if self.register_a != 0 {
                        self.ip = operand as usize;
                    }
                }
                // bxc
                0x4 => {
                    self.register_b = self.register_b ^ self.register_c;
                }
                // out
                0x5 => {
                    let actual_operand = self.combo(operand);
                    self.output((actual_operand % 8) as u8);
                }
                // bdv
                0x6 => {
                    let numerator = self.register_a;
                    let actual_operand = self.combo(operand);
                    let divisor = 1 << actual_operand;
                    self.register_b = numerator / divisor;
                }
                // cdv
                0x7 => {
                    let numerator = self.register_a;
                    let actual_operand = self.combo(operand);
                    let divisor = 1 << actual_operand;
                    self.register_c = numerator / divisor;
                }
                _ => unreachable!(),
            }

            if let Some(_) = cb(self) {
            } else {
                break;
            }
        }
    }

    fn set_a(&mut self, value: u128) {
        self.register_a = value;
    }

    fn set_b(&mut self, value: u128) {
        self.register_b = value;
    }

    fn set_c(&mut self, value: u128) {
        self.register_c = value;
    }

    fn combo(&self, op: Tryte) -> u128 {
        match op {
            v @ 0..=3 => v as u128,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            7 => unreachable!(),
            _ => unreachable!(),
        }
    }

    fn output(&mut self, value: u8) {
        self.stdout.push(value);
    }
}

fn main() {
    let src = include_str!("src2.txt");

    //     let src = "Register A: 729
    // Register B: 0
    // Register C: 0

    // Program: 0,1,5,4,3,0";

    //     let src = "Register A: 2024
    // Register B: 0
    // Register C: 0

    // Program: 0,3,5,4,3,0";

    let mut lines = src.lines();
    let _ = lines.next();
    let reg_b = lines
        .next()
        .unwrap()
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .parse::<u128>()
        .unwrap();
    let reg_c = lines
        .next()
        .unwrap()
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .parse::<u128>()
        .unwrap();

    let program = lines
        .skip(1)
        .next()
        .unwrap()
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|s| s.parse::<u8>().unwrap())
        .collect::<Vec<_>>();

    let mut comp = Computer::new(&program);

    // states are (a, i)
    let mut states = vec![(0, 0)];
    let mut res = vec![];
    while let Some((a, i)) = states.pop() {
        if i == program.len() {
            res.push(a);
            continue;
        }
        for b in 0..8 {
            let new_a = (a << 3) | b;
            let target = program[program.len() - i - 1];
            if first_output(&mut comp, new_a) == target {
                states.push((new_a, i + 1));
            }
        }
    }
    println!("{}", res.into_iter().min().unwrap());
}

fn first_output(comp: &mut Computer, a: u128) -> u8 {
    comp.reset();
    comp.set_a(a);
    comp.run(|_| Some(()));
    comp.stdout[0]
}
