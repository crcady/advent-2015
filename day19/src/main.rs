use std::{
    collections::HashSet,
    io::{self, BufRead},
};

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

    let min_steps = depth_first_search(&rules, &mut input.to_string(), 0, None);
    println!("Best I can do is {} steps", min_steps.unwrap_or(0));
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

fn depth_first_search(
    rules: &Vec<Replacement>,
    the_string: &mut String,
    call_depth: usize,
    stop_depth: Option<usize>,
) -> Option<usize> {
    if let Some(stop_depth) = stop_depth {
        if call_depth >= stop_depth {
            return None;
        }
    }

    if the_string == "e" {
        return Some(call_depth);
    }

    let mut best_so_far: Option<usize> = stop_depth;

    for rule in rules {
        let re = Regex::new(&rule.to).unwrap();
        let ranges: Vec<_> = re
            .find_iter(&the_string)
            .map(|x| (x.start(), x.end()))
            .collect();

        for r in ranges.iter() {
            // Do the replacement
            the_string.replace_range(r.0..r.1, &rule.from);

            // Call ourselves recursively
            let res = depth_first_search(rules, the_string, call_depth + 1, best_so_far);

            // Check the result
            if let Some(found_steps) = res {
                if let Some(previous_best) = best_so_far {
                    if found_steps < previous_best {
                        dbg!(found_steps);
                        best_so_far = Some(found_steps);
                    }
                } else {
                    dbg!(found_steps);
                    best_so_far = Some(found_steps);
                }
            }

            // Undo the replacement
            the_string.replace_range(r.0..r.0 + rule.from.len(), &rule.to);
        }
    }

    best_so_far
}
