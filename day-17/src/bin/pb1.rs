type Tryte = u8;

struct Computer {
    program: Vec<Tryte>,
    register_a: i64,
    register_b: i64,
    register_c: i64,
    ip: usize,
    stdout: Vec<i64>,
}

impl Computer {
    fn new(program: impl Into<Vec<Tryte>>) -> Self {
        Self {
            program: program.into(),
            register_a: 0,
            register_b: 0,
            register_c: 0,
            ip: 0,
            stdout: Vec::new(),
        }
    }

    pub fn run(&mut self, cb: fn(&Self)) {
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
                    let right = operand as i64;
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
                    self.output(actual_operand % 8);
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

            cb(self);
        }
    }

    fn set_a(&mut self, value: i64) {
        self.register_a = value;
    }

    fn set_b(&mut self, value: i64) {
        self.register_b = value;
    }

    fn set_c(&mut self, value: i64) {
        self.register_c = value;
    }

    fn combo(&self, op: Tryte) -> i64 {
        match op {
            v @ 0..=3 => v as i64,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            7 => unreachable!(),
            _ => unreachable!(),
        }
    }

    fn output(&mut self, value: i64) {
        self.stdout.push(value);
    }
}

fn main() {
    let src = include_str!("src1.txt");

//     let src = "Register A: 729
// Register B: 0
// Register C: 0

// Program: 0,1,5,4,3,0";

    let mut lines = src.lines();
    let reg_a = lines
        .next()
        .unwrap()
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .parse::<i64>()
        .unwrap();
    let reg_b = lines
        .next()
        .unwrap()
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .parse::<i64>()
        .unwrap();
    let reg_c = lines
        .next()
        .unwrap()
        .split(':')
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .parse::<i64>()
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

    let mut computer = Computer::new(program);
    computer.set_a(reg_a);
    computer.set_b(reg_b);
    computer.set_c(reg_c);
    computer.run(|c| {});
    println!(
        "{}",
        computer
            .stdout
            .iter()
            .map(|v| format!("{v}"))
            .collect::<Vec<_>>()
            .join(",")
    );
}
