use std::{
    cmp::min,
    io::{self, BufRead},
};

fn main() {
    let mut numbers: Vec<i32> = Vec::new();

    for line in io::stdin().lock().lines() {
        numbers.push(line.unwrap().parse().unwrap());
    }

    println!("There are {} ways to make 150", count_ways(150, &numbers));

    let all_the_ways = enumerate_ways(150, &numbers);

    let mut min_containers: Option<u32> = None;

    for way in &all_the_ways {
        let mut count = 0;
        for v in way {
            if *v {
                count += 1;
            }
        }
        if let Some(current) = min_containers {
            min_containers = Some(min(current, count));
        } else {
            min_containers = Some(count);
        }
    }

    let mut way_count = 0;

    for way in &all_the_ways {
        let mut count = 0;
        for v in way {
            if *v {
                count += 1;
            }
        }

        if count == min_containers.unwrap() {
            way_count += 1;
        }
    }

    println!(
        "You need at least {} containers and you can do that {} ways",
        min_containers.unwrap(),
        way_count
    );
}

fn count_ways(remaining: i32, buckets: &Vec<i32>) -> i32 {
    if remaining < 0 {
        return 0;
    }

    if remaining == 0 {
        return 1;
    }

    if buckets.len() == 0 {
        return 0;
    }

    let mut res = 0i32;
    let mut it = buckets.iter();

    let current = it.next().unwrap();

    let new_buckets = Vec::from_iter(it);
    let new_buckets = new_buckets.into_iter().map(|x| *x).collect(); // There's gotta be an eaasier way...

    res += count_ways(remaining, &new_buckets);
    res += count_ways(remaining - current, &new_buckets);

    res
}

fn enumerate_ways(remaining: i32, buckets: &Vec<i32>) -> Vec<Vec<bool>> {
    if remaining < 0 {
        return Vec::new();
    }

    if remaining == 0 {
        return vec![vec![false; buckets.len()]];
    }

    if buckets.len() == 0 {
        return Vec::new();
    }

    let mut res: Vec<Vec<bool>> = Vec::new();
    let mut it = buckets.iter();

    let current = it.next().unwrap();

    let new_buckets = Vec::from_iter(it);
    let new_buckets = new_buckets.into_iter().map(|x| *x).collect(); // There's gotta be an eaasier way...

    let results_without = enumerate_ways(remaining, &new_buckets);
    let results_with = enumerate_ways(remaining - current, &new_buckets);

    for ending in results_without {
        res.push([vec![false], ending].concat());
    }

    for ending in results_with {
        res.push([vec![true], ending].concat());
    }

    res
}
