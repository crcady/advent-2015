use std::io::{self, BufRead};

fn main() {
    let mut instrs: Vec<Instruction> = Vec::new();

    for line in io::stdin().lock().lines() {
        instrs.push(parse_line(&line.unwrap()));
    }

    let mut proc = Processor {
        a: 0,
        b: 0,
        ip: 0,
        instrs
    };

    while proc.step() {}

    println!("Finished with a: {} and b: {}", proc.a, proc.b);
}

struct Processor {
    a: u32,
    b: u32,
    ip: usize,

    instrs: Vec<Instruction>,
}
#[allow(non_camel_case_types)]
enum Register {
    a,
    b,
}
#[allow(non_camel_case_types)]
enum Instruction {
    hlf(Register),
    tpl(Register),
    inc(Register),
    jmp(i32),
    jie(Register, i32),
    jio(Register, i32),
}

impl Register {
    fn from_str(s: &str) -> Self {
        match s {
            "a" => Register::a,
            "b" => Register::b,
            _ => unimplemented!(),
        }
    }
}

impl Processor {
    fn step(&mut self) -> bool {
        match &self.instrs[self.ip] {
            Instruction::hlf(register) => {
                match register {
                    Register::a => {self.a >>= 1;},
                    Register::b => {self.b >>= 1;},
                }
                self.ip += 1;
            },
            Instruction::tpl(register) => {
                match register {
                    Register::a => {self.a *= 3;},
                    Register::b => {self.b *= 3;},
                }
                self.ip += 1;
            },
            Instruction::inc(register) => {
                match register {
                    Register::a => {self.a += 1;},
                    Register::b => {self.b += 1;},
                }
                self.ip += 1;
            },
            Instruction::jmp(offset) => {
                self.ip = (self.ip as i32 + offset).try_into().unwrap();
            },
            Instruction::jie(register, offset) => {
                let rval = match register {
                    Register::a => self.a,
                    Register::b => self.b,
                };
                if rval % 2 == 0 {
                    self.ip = (self.ip as i32 + offset).try_into().unwrap();
                } else {
                    self.ip += 1;
                }
            },
            Instruction::jio(register, offset) => {
                let rval = match register {
                    Register::a => self.a,
                    Register::b => self.b,
                };
                if rval == 1 {
                    self.ip = (self.ip as i32 + offset).try_into().unwrap();
                } else {
                    self.ip += 1;
                }
            },
        }
        self.ip < self.instrs.len()
    }
}

fn parse_line(line: &str) -> Instruction {
    let splits: Vec<_> = line.split(" ").map(|x|x.trim().trim_end_matches(",")).collect();
    match splits[0] {
        "hlf" => Instruction::hlf(Register::from_str(splits[1])),
        "tpl" => Instruction::tpl(Register::from_str(splits[1])),
        "inc" => Instruction::inc(Register::from_str(splits[1])),
        "jmp" => Instruction::jmp(splits[1].parse().unwrap()),
        "jie" => Instruction::jie(Register::from_str(splits[1]), splits[2].parse().unwrap()),
        "jio" => Instruction::jio(Register::from_str(splits[1]), splits[2].parse().unwrap()),
        _ => unimplemented!(),
    }
}