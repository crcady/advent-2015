use std::{collections::HashSet, io};


fn main() {
    println!("What's your current password?");

    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);

    let mut password: Vec<char> = input.trim().chars().collect();

    let mut good = false;

    while !good {
        // Roll over to the next password
        for j in 0..password.len() {
            let i = password.len() - j - 1;

            match password[i] {
                'z' => {password[i] = 'a';},
                other => { password[i] = std::char::from_u32(other as u32 + 1).unwrap(); break; },
            }
        }
        good = true;
        // Check each of the three rules
        
        // Increasing straight
        good = good && straight_of_three(&password);
        
        // No i, o, or l
        good = good && no_confusing_chars(&password);

        // At least 2 different, non-overlapping pairs
        good = good && two_pair(&password);
    }
    let pw_string: String = password.into_iter().collect();
    println!("Your new password is: {}", pw_string);
}

fn no_confusing_chars(pw: &Vec<char>) -> bool {
    for c in pw {
        match c {
            'i' => return false,
            'o' => return false,
            'l' => return false,
            _ => (),
        }
    }
    true
}

fn straight_of_three(pw: &Vec<char>) -> bool {
    let pw_as_ints: Vec<u32> = pw.iter().map(|x| *x as u32).collect();

    let mut two_ago = pw_as_ints[0];
    let mut one_ago = pw_as_ints[1];

    for i in 2..pw_as_ints.len() {
        let current = pw_as_ints[i];
        if current == one_ago + 1 && one_ago == two_ago + 1 {
            return true
        }

        two_ago = one_ago;
        one_ago = current;
    }
    false
}

fn two_pair(pw: &Vec<char>) -> bool {
    let mut last = pw[0];
    let mut pairs: HashSet<char> = HashSet::new();

    for i in 1..pw.len() {
        let current = pw[i];
        if current == last {
            pairs.insert(current);
        }
        last = current;
    }

    pairs.len() >= 2
}