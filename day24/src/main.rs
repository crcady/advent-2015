use std::{collections::HashSet, io::{self, BufRead}};

fn main() {
    let mut nums: Vec<u64> = Vec::new();

    for line in io::stdin().lock().lines() {
        nums.push(line.unwrap().parse().unwrap());
    }

    let sum:u64 = nums.iter().sum();

    if sum % 4 != 0 {
        println!("The sum ({}) is not divisible by four!", sum);
        panic!()
    }

    let target_value = sum / 4;

    let mut combos = find_combos(&nums, target_value);
    println!("Found {} combinations that add up to {}", combos.len(), target_value);
    combos.sort_by_key(|x|x.len());
    let mut viable_length: Option<usize> = None;
    let mut min_qe: Option<u64> = None;

    for i in 0..combos.len() {
        let current = &combos[i];
        if let Some(vl) = viable_length {
            if current.len() > vl {
                break;
            }
        }

        let mut all_numbers: HashSet<u64> = HashSet::from_iter(nums.iter().map(|x|*x));
        for num in current {
            all_numbers.remove(num);
        }

        let remaining_numbers: Vec<u64> = Vec::from_iter(all_numbers.iter().map(|x| *x));
        let other_combos = find_combos(&remaining_numbers, target_value);
        if other_combos.len() == 0 {
            continue;
        }
        let mut viable = false;
        for vec in &other_combos {
            let mut all_numbers: HashSet<u64> = HashSet::from_iter(nums.iter().map(|x|*x));
            for num in current {
                all_numbers.remove(num);
            }

            for num in vec {
                all_numbers.remove(num);
            }

            let remaining_numbers: Vec<u64> = Vec::from_iter(all_numbers.iter().map(|x| *x));
            if quick_check(&remaining_numbers, target_value) {
                viable = true;
                break;
            }
        }

        if !viable {
            continue;
        }

        // If we got here, we have a viable first group
        viable_length = Some(current.len());
        let new_qe: u64 = current.iter().product();
        if let Some(qe) = min_qe {
            if new_qe < qe {
                min_qe = Some(new_qe);
            }
        } else {
            min_qe = Some(new_qe);
        }

    }

    println!("Including the trunk, the minimum QE is {}", min_qe.unwrap());



    if sum % 3 != 0 {
        println!("The sum ({}) is not divisible by three!", sum);
        panic!()
    }

    let target_value = sum / 3;

    let combos = find_combos(&nums, target_value);
    println!("Found {} combinations that add up to {}", combos.len(), target_value);
    let mut combos = check_viable(&nums, target_value, combos);
    println!("{} of them are viable", combos.len());

    combos.sort_by_key(|x|x.len());
    let min_packages = combos[0].len();
    println!("Looking at combinations of {} packages", min_packages);

    let mut min_qe: u64 = combos[0].iter().product();

    println!("The first set has a QE of {}", min_qe);
    for i in 1..combos.len() {
        if combos[i].len() > min_packages {
            break;
        }
        let new_qe: u64 = combos[i].iter().product();
        if new_qe < min_qe {
            min_qe = new_qe;
        }
    }
    println!("The minimum quantum entanglement is {}", min_qe);

}

fn find_combos(nums: &Vec<u64>, target_value: u64) -> Vec<Vec<u64>> {
    // If we hit the target value exactly, just return a single, empty vector
    if target_value == 0 {
        return vec![Vec::new()]
    }

    // If there are no numbers to choose from and we have a nonzero target, we can't get there from here. Return an empty vector.
    if nums.len() == 0 {
        return Vec::new();
    }

    // Quit early if we don't have enough remaining
    if target_value > nums.iter().sum() {
        return Vec::new();
    }

    let mut it = nums.iter();
    let first = *it.next().unwrap();

    let remaining: Vec<u64> = Vec::from_iter(it.map(|x| *x));
    let mut res: Vec<Vec<u64>> = Vec::new();

    if first <= target_value {
        for vec in find_combos(&remaining, target_value - first) {
            let mut new_vec = vec![first];
            new_vec.extend(vec);
            res.push(new_vec);
        }
    }

    // The other case is much simpler
    res.extend(find_combos(&remaining, target_value));

    res
}

fn check_viable(nums: &Vec<u64>, target_value: u64, combos: Vec<Vec<u64>>) -> Vec<Vec<u64>> {
    let mut res: Vec<Vec<u64>> = Vec::new();
    for vec in combos {
        let mut remaining_numbers: HashSet<u64> = HashSet::from_iter(nums.iter().map(|x| *x));
        for num in &vec {
            remaining_numbers.remove(num);
        }
        let remaining_numbers: Vec<u64> = Vec::from_iter(remaining_numbers.iter().map(|x| *x));
        if quick_check(&remaining_numbers, target_value) {
            res.push(vec);
        }
    }
    res
}

fn quick_check(nums: &[u64], target_value: u64) -> bool {
    if target_value == 0 {
        return true;
    }

    if target_value > nums.iter().sum() {
        return false;
    }

    if nums[0] > target_value {
        return quick_check(&nums[1..], target_value);
    }

    quick_check(&nums[1..], target_value - nums[0]) || quick_check(&nums[1..], target_value)

}