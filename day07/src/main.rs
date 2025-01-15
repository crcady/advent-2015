use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    println!("Give me a circuit, followed by ctrl-d");

    let mut concrete: HashMap<String, u16> = HashMap::new();
    let mut symbolic: HashMap<String, Gate> = HashMap::new();

    // concrete.insert("A", 0xFF);
    // concrete.insert("B", 0xA5);

    // symbolic.insert(
    //     "C",
    //     Gate::And {
    //         left: "A",
    //         right: "B",
    //     },
    // );

    let lookup = |name: &str| -> Option<u16> {
        match concrete.get(name) {
            Some(val) => Some(*val),
            None => match name.parse::<u16>() {
                Ok(val) => Some(val),
                _ => None,
            },
        }
    };

    for line in io::stdin().lock().lines() {
        let line = line.expect("Didn't get a line");
        let (gate, name) = to_gate(&line);

        match gate {
            Gate::Concrete(val) => {
                concrete.insert(name.to_string(), val);
            }
            other_gate => {
                symbolic.insert(name.to_string(), other_gate);
            }
        };
    }
    println!("Solving...");

    let mut to_remove: Option<String> = None;

    while symbolic.len() > 0 {
        if let Some(name) = to_remove {
            symbolic.remove(&name);
            to_remove = None;
        }

        println!("Symbolic: {}, Concrete: {}", symbolic.len(), concrete.len());
        for (k, v) in concrete.iter() {
            println!("{}: {}", k, v)
        }

        'outer: for (name, gate) in symbolic.iter() {
            match gate {
                Gate::Concrete(_) => (), // This shouldn't ever happen, since we're using two HashMaps
                Gate::And { left, right } => {
                    if let Some(left_val) = concrete.get(left) {
                        if let Some(right_val) = concrete.get(right) {
                            concrete.insert(name.to_string(), left_val & right_val);
                            to_remove = Some(name.to_string());
                            break 'outer;
                        }
                    }
                }
                Gate::Or { left, right } => {
                    if let Some(left_val) = concrete.get(left) {
                        if let Some(right_val) = concrete.get(right) {
                            concrete.insert(name.to_string(), left_val | right_val);
                            to_remove = Some(name.to_string());
                            break 'outer;
                        }
                    }
                }
                Gate::ShiftL { left, right } => {
                    if let Some(left_val) = concrete.get(left) {
                        if let Ok(right_val) = right.parse::<u16>() {
                            concrete.insert(name.to_string(), left_val << right_val);
                            to_remove = Some(name.to_string());
                            break 'outer;
                        }
                    }
                }
                Gate::ShiftR { left, right } => {
                    if let Some(left_val) = concrete.get(left) {
                        if let Ok(right_val) = right.parse::<u16>() {
                            concrete.insert(name.to_string(), left_val >> right_val);
                            to_remove = Some(name.to_string());
                            break 'outer;
                        }
                    }
                }
                Gate::Not { right } => {
                    if let Some(right_val) = concrete.get(right) {
                        concrete.insert(name.to_string(), !right_val);
                        to_remove = Some(name.to_string());
                        break 'outer;
                    }
                }
            }
        }
    }

    for (k, v) in concrete.iter() {
        println!("{}: {}", k, v)
    }

    match concrete.get("a") {
        Some(v) => println!("a = {}", v),
        None => println!("a not found!"),
    }
}

enum Gate {
    Concrete(u16),
    And { left: String, right: String },
    Or { left: String, right: String },
    ShiftL { left: String, right: String },
    ShiftR { left: String, right: String },
    Not { right: String },
}

fn to_gate<'a>(line: &'a String) -> (Gate, &str) {
    let split: Vec<&str> = line.split("->").collect();
    let name = split[1].trim();
    let split: Vec<&str> = split[0].trim().split(" ").collect();
    match split.len() {
        1 => (
            Gate::Concrete(
                split[0]
                    .parse()
                    .expect("Couldn't parse concrete value in gate definition"),
            ),
            &name,
        ),
        2 => (
            Gate::Not {
                right: split[1].to_string(),
            },
            &name,
        ),
        3 => (
            match split[1] {
                "AND" => Gate::And {
                    left: split[0].to_string(),
                    right: split[2].to_string(),
                },
                "OR" => Gate::Or {
                    left: split[0].to_string(),
                    right: split[2].to_string(),
                },
                "LSHIFT" => Gate::ShiftL {
                    left: split[0].to_string(),
                    right: split[2].to_string(),
                },
                "RSHIFT" => Gate::ShiftR {
                    left: split[0].to_string(),
                    right: split[2].to_string(),
                },
                _ => panic!("Unknown gate type"),
            },
            &name,
        ),
        _ => panic!("Unexpected number of splits in line {}: {}", line, split[3]),
    }
}
