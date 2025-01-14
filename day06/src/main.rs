use std::{io, io::BufRead};

fn main() {
    println!("Enter some instructions, then ctrl-d");
    let mut lg = LightGrid::new();
    let mut fl = FancyLights::new();

    for line in io::stdin().lock().lines() {
        let ins = parse_line(&line.unwrap());
        match ins.op {
            Operation::Toggle => {
                lg.toggle(ins.x0, ins.y0, ins.x1, ins.y1);
                fl.toggle(ins.x0, ins.y0, ins.x1, ins.y1)
            }
            Operation::TurnOn => {
                lg.turn_on(ins.x0, ins.y0, ins.x1, ins.y1);
                fl.turn_on(ins.x0, ins.y0, ins.x1, ins.y1)
            }
            Operation::TurnOff => {
                lg.turn_off(ins.x0, ins.y0, ins.x1, ins.y1);
                fl.turn_off(ins.x0, ins.y0, ins.x1, ins.y1)
            }
        }
    }

    println!("There are {} lights on", lg.count());
    println!("The intensity is {}", fl.count());
}

enum Operation {
    Toggle,
    TurnOn,
    TurnOff,
}

struct Instr {
    op: Operation,
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
}

fn parse_line(line: &str) -> Instr {
    let words: Vec<&str> = line.split(' ').collect();
    let mut op: Operation = Operation::Toggle;

    if words[0] != "toggle" {
        if words[1] == "on" {
            op = Operation::TurnOn;
        } else {
            op = Operation::TurnOff;
        }
    }

    let first_pair: Vec<&str> = match op {
        Operation::Toggle => words[1],
        _ => words[2],
    }
    .split(',')
    .collect();

    let second_pair: Vec<&str> = match op {
        Operation::Toggle => words[3],
        _ => words[4],
    }
    .split(',')
    .collect();

    let x0: usize = first_pair[0].parse().unwrap();
    let y0: usize = first_pair[1].parse().unwrap();

    let x1: usize = second_pair[0].parse().unwrap();
    let y1: usize = second_pair[1].parse().unwrap();

    Instr { op, x0, y0, x1, y1 }
}

struct LightGrid {
    lights: Vec<Vec<bool>>,
}

impl LightGrid {
    fn new() -> LightGrid {
        LightGrid {
            lights: vec![vec![false; 1000]; 1000],
        }
    }

    fn count(&self) -> u32 {
        let mut count: u32 = 0;
        for row in &self.lights {
            for light in row {
                if *light {
                    count += 1;
                }
            }
        }
        count
    }

    fn toggle(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        for x in x0..=x1 {
            for y in y0..=y1 {
                self.lights[x][y] = !self.lights[x][y]
            }
        }
    }

    fn turn_on(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        for x in x0..=x1 {
            for y in y0..=y1 {
                self.lights[x][y] = true;
            }
        }
    }

    fn turn_off(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        for x in x0..=x1 {
            for y in y0..=y1 {
                self.lights[x][y] = false;
            }
        }
    }
}

struct FancyLights {
    lights: Vec<Vec<u32>>,
}

impl FancyLights {
    fn new() -> FancyLights {
        FancyLights {
            lights: vec![vec![0; 1000]; 1000],
        }
    }

    fn count(&self) -> u32 {
        let mut count: u32 = 0;
        for row in &self.lights {
            for light in row {
                count += *light;
            }
        }
        count
    }

    fn toggle(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        for x in x0..=x1 {
            for y in y0..=y1 {
                self.lights[x][y] += 2;
            }
        }
    }

    fn turn_on(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        for x in x0..=x1 {
            for y in y0..=y1 {
                self.lights[x][y] += 1;
            }
        }
    }

    fn turn_off(&mut self, x0: usize, y0: usize, x1: usize, y1: usize) {
        for x in x0..=x1 {
            for y in y0..=y1 {
                if self.lights[x][y] > 0 {
                    self.lights[x][y] -= 1;
                }
            }
        }
    }
}
