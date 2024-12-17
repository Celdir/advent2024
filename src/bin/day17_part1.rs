use std::io::{stdin, BufRead};

#[derive(Debug)]
struct Computer {
    a: usize,
    b: usize,
    c: usize,
    program: Vec<usize>,
    instruction: usize,
    output: Vec<usize>,
}

impl Computer {
    fn new(a: usize, b: usize, c: usize, program: Vec<usize>) -> Self {
        Computer {
            a,
            b,
            c,
            program,
            instruction: 0,
            output: Vec::new(),
        }
    }

    fn run(&mut self) -> String {
        while self.instruction < self.program.len() {
            let (opcode, operand) = (
                self.program[self.instruction],
                self.program[self.instruction + 1],
            );
            match opcode {
                0 => self.adv(operand),
                1 => self.bxl(operand),
                2 => self.bst(operand),
                3 => self.jnz(operand),
                4 => self.bxc(operand),
                5 => self.out(operand),
                6 => self.bdv(operand),
                7 => self.cdv(operand),
                _ => panic!("invalid opcode"),
            };
        }
        self.output
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn adv(&mut self, operand: usize) {
        self.a /= 2usize.pow(self.combo(operand) as u32);
        self.instruction += 2;
    }
    fn bxl(&mut self, operand: usize) {
        self.b ^= operand;
        self.instruction += 2;
    }
    fn bst(&mut self, operand: usize) {
        self.b = self.combo(operand) % 8;
        self.instruction += 2;
    }
    fn jnz(&mut self, operand: usize) {
        if self.a == 0 {
            self.instruction += 2;
            return;
        }
        self.instruction = operand;
    }
    fn bxc(&mut self, _: usize) {
        self.b ^= self.c;
        self.instruction += 2;
    }
    fn out(&mut self, operand: usize) {
        self.output.push(self.combo(operand) % 8);
        self.instruction += 2;
    }
    fn bdv(&mut self, operand: usize) {
        self.b = self.a / 2usize.pow(self.combo(operand) as u32);
        self.instruction += 2;
    }
    fn cdv(&mut self, operand: usize) {
        self.c = self.a / 2usize.pow(self.combo(operand) as u32);
        self.instruction += 2;
    }

    fn combo(&self, operand: usize) -> usize {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("invalid operand"),
        }
    }
}

fn main() {
    let input: Vec<String> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| !l.is_empty())
        .collect();
    let registers: Vec<usize> = input[0..3]
        .iter()
        .map(|s| s.split_once(": ").unwrap().1.parse().unwrap())
        .collect();
    let (&a, &b, &c) = match &registers[..] {
        [a, b, c] => (a, b, c),
        _ => panic!("there should only be 3 registers"),
    };
    let program: Vec<usize> = input[3]
        .split_once(": ")
        .unwrap()
        .1
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    let mut computer = Computer::new(a, b, c, program);
    println!("{:?}", computer);
    println!("{}", computer.run());
}
