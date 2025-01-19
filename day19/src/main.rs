use std::{collections::HashSet, io::{self, BufRead}};

use regex::Regex;

fn main() {
    let mut input = String::new();
    let mut rules: Vec<Replacement> = Vec::new();

    let mut io_lock = io::stdin().lock();
    let mut buf = String::new();

    loop {
        buf.clear();
        let res = io_lock.read_line(&mut buf);
        match res {
            Ok(byte_count) => match byte_count {
                1 => {
                    let _ = io_lock.read_line(&mut input);
                    break;
                }
                _ => {
                    rules.push(parse_line(&buf.trim()));
                }
            },
            _ => unimplemented!(), // Don't think we should hit this
        }
    }

    let input = input.trim();

    let mut possibilities: HashSet<String> = HashSet::new();

    for rule in &rules {
        for new_string in build_outputs(rule, &input).into_iter() {
            // It's okay to move and consume the returned vector here, we don't need it
            possibilities.insert(new_string);
        }
    }

    println!("Found {} unique options", possibilities.len());

    let mut steps = 0;
    possibilities.clear();
    possibilities.insert("e".to_owned());

    loop {
        match possibilities.get(input) {
            Some(_) => break,
            None => {
                let mut new_possibilities: HashSet<String> = HashSet::new();
                for line in &possibilities {
                    for rule in &rules {
                        for new_string in build_outputs(rule, line).into_iter() {
                            if input.len() >= new_string.len() {
                                new_possibilities.insert(new_string);
                            }
                        }
                    }
                }
                possibilities = new_possibilities;
                steps += 1;
            },
        }
    }

    println!("Took {} steps to find the molecule", steps);


}

#[derive(Debug)]
struct Replacement {
    from: String,
    to: String,
}

fn build_outputs(rep: &Replacement, line: &str) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let re = Regex::new(&rep.from).unwrap();
    for m in re.find_iter(line) {
        res.push(line[0..m.start()].to_string() + &rep.to + &line[m.end()..]);
    }
    res
}

fn parse_line(line: &str) -> Replacement {
    let splits: Vec<&str> = line.split(" ").collect();

    Replacement {
        from: splits[0].to_owned(),
        to: splits[2].to_owned(),
    }
}
