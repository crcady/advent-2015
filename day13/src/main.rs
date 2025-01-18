use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

fn main() {
    println!("Enter cities in the format\nAlice would gain 54 happiness units by sitting next to Bob.\nctrl-d when done");
    let mut edges: Vec<Edge> = Vec::new();

    for line in io::stdin().lock().lines() {
        let new_edge = parse_line(&line.unwrap().trim());
        edges.push(new_edge);
    }

    println!("Got {} edges, checking for guests", edges.len());

    let mut guests: HashSet<String> = HashSet::new();

    for edge in &edges {
        guests.insert(edge.src.to_string());
        guests.insert(edge.dst.to_string());
    }

    // Uncomment this line to solve part 2
    // guests.insert("Camdon".to_owned());

    println!("Found {} guests", guests.len());

    let guests: Vec<String> = guests.iter().map(|x| x.to_string()).collect(); // TODO: Extra allocation?
    let mut indices: HashMap<String, usize> = HashMap::new();

    for i in 0..guests.len() {
        indices.insert(guests[i].to_string(), i);
    }

    let mut deltas: Vec<Vec<i32>> = vec![vec!(0; guests.len()); guests.len()];
    for edge in &edges {
        let i = *indices.get(&edge.src).unwrap();
        let j = *indices.get(&edge.dst).unwrap();
        deltas[i][j] = edge.delta;
    }

    let mut all_guests: Vec<usize> = indices.values().map(|x|*x).collect();
    let start_guest = all_guests.pop().unwrap();

    let max_delta = badly_recursive_search(
        &deltas,
        start_guest,
        &all_guests,
        None,
        max,
    );

    println!("Maximum delta found: {}", max_delta);
}

fn badly_recursive_search(
    deltas: &Vec<Vec<i32>>,
    start: usize,
    remaining_guests: &Vec<usize>,
    from: Option<usize>,
    cmp: fn(i32, i32) -> i32,
) -> i32 {
    if remaining_guests.len() == 0 {
        return deltas[from.unwrap()][start] + deltas[start][from.unwrap()]; // It would be an error to have no guests and no from
    }

    let mut cmp_found: Option<i32> = None;
    let from = match from {
        Some(f) => Some(f),
        _ => Some(start),
    };

    for next_guest in remaining_guests {
        let mut new_remaining: Vec<usize> = Vec::new();
        for i in remaining_guests {
            if i == next_guest {
                continue;
            }

            new_remaining.push(*i);
        }
        let arrival_cost: i32 = match from {
            Some(from_guest) => deltas[from_guest][*next_guest] + deltas[*next_guest][from_guest],
            _ => unimplemented!(),
        };
        let found_result =
            badly_recursive_search(deltas, start, &new_remaining, Some(*next_guest), cmp)
                + arrival_cost;
        cmp_found = match cmp_found {
            Some(current) => Some(cmp(current, found_result)),
            _ => Some(found_result),
        }
    }

    return cmp_found.unwrap();
}

fn parse_line(line: &str) -> Edge {
    // The format of the line is:
    // Alice would gain 54 happiness units by sitting next to Bob.
    //  [0]   [1]  [2] [3]    [4]     [5] [6]   [7]   [8] [9] [10]

    let splits: Vec<&str> = line.split(" ").collect();
    let delta: i32 = match splits[2] {
        "gain" => splits[3]
            .parse()
            .expect("Couldn't parse the gain to a number"),
        "lose" => -splits[3]
            .parse::<i32>()
            .expect("Couldn't parse the loss to a number"),
        _ => panic!("Coudn't parse gain/loss"),
    };
    Edge {
        src: splits[0].to_string(),
        dst: splits[10][0..splits[10].len() - 1].to_string(),
        delta: delta,
    }
}

#[derive(Debug)]
struct Edge {
    src: String,
    dst: String,
    delta: i32,
}
