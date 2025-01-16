use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

fn main() {
    println!("Enter cities in the format\nPhoenix to Prescott = 100\nctrl-d when done");
    let mut edges: Vec<Edge> = Vec::new();

    for line in io::stdin().lock().lines() {
        let new_edge = parse_line(&line.unwrap().trim());
        edges.push(new_edge);
    }

    println!("Got {} edges, checking for cities", edges.len());

    let mut cities: HashSet<String> = HashSet::new();

    for edge in &edges {
        cities.insert(edge.src.to_string()); // Don't want cities to own the edge
        cities.insert(edge.dst.to_string());
    }

    println!("Found {} cities", cities.len());

    let cities: Vec<String> = cities.iter().map(|x| x.to_string()).collect(); // TODO: Extra allocation?
    let mut indices: HashMap<String, usize> = HashMap::new();

    for i in 0..cities.len() {
        indices.insert(cities[i].to_string(), i);
    }

    let mut distances: Vec<Vec<u32>> = vec![vec!(0; cities.len()); cities.len()];
    for edge in &edges {
        let i = *indices.get(&edge.src).unwrap();
        let j = *indices.get(&edge.dst).unwrap();
        distances[i][j] = edge.distance;
        distances[j][i] = edge.distance;
    }

    let min_distance = badly_recursive_search(
        &distances,
        &indices.values().map(|x| *x).collect(),
        None,
        min,
    );
    let max_distance = badly_recursive_search(
        &distances,
        &indices.values().map(|x| *x).collect(),
        None,
        max,
    );

    println!("Minimum distance found: {}", min_distance);
    println!("Maximum distance found: {}", max_distance);
}

fn badly_recursive_search(
    distances: &Vec<Vec<u32>>,
    remaining_cities: &Vec<usize>,
    from: Option<usize>,
    cmp: fn(u32, u32) -> u32,
) -> u32 {
    if remaining_cities.len() == 1 {
        return distances[remaining_cities[0]][from.unwrap()]; // It would be an error to only have one city and no from
    }

    let mut min_found: Option<u32> = None;

    for next_city in remaining_cities {
        let mut new_remaining: Vec<usize> = Vec::new();
        for i in remaining_cities {
            if i == next_city {
                continue;
            }

            new_remaining.push(*i);
        }
        let arrival_cost: u32 = match from {
            Some(from_city) => distances[from_city][*next_city],
            _ => 0,
        };
        let found_result =
            badly_recursive_search(distances, &new_remaining, Some(*next_city), cmp) + arrival_cost;
        min_found = match min_found {
            Some(current) => Some(cmp(current, found_result)),
            _ => Some(found_result),
        }
    }

    return min_found.unwrap();
}

fn parse_line(line: &str) -> Edge {
    // The format of the line is:
    // Source to Destination = Distance

    let splits: Vec<&str> = line.split(" ").collect();
    Edge {
        src: splits[0].to_string(),
        dst: splits[2].to_string(),
        distance: splits[4].parse().expect("Couldn't parse distance"),
    }
}

#[derive(Debug)]
struct Edge {
    src: String,
    dst: String,
    distance: u32,
}
