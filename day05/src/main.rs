use std::{collections::HashSet, io, io::BufRead};

fn main() {
    println!("Give me some strings, I'll tell you if they're naughty or nice!");

    let mut nice_count = 0;
    let mut nice_count2 = 0;

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let nice = check_vowels(&line) && check_repeated(&line) && check_banned(&line);
        let nice2 = check_pairs(&line) && check_repeated2(&line);

        if nice {
            nice_count += 1;
        }

        if nice2 {
            nice_count2 += 1;
        }
    }

    println!("Found {} and then {} nice strings", nice_count, nice_count2);
}

fn check_vowels(line: &str) -> bool {
    let mut count = 0;
    for c in line.chars() {
        match c {
            'a' => count += 1,
            'e' => count += 1,
            'i' => count += 1,
            'o' => count += 1,
            'u' => count += 1,
            _ => (),
        }
    }
    return count >= 3;
}

fn check_repeated(line: &str) -> bool {
    let mut last: char = ' ';
    for c in line.chars() {
        if c == last {
            return true;
        }
        last = c;
    }
    return false;
}

fn check_banned(line: &str) -> bool {
    let mut last: char = ' ';
    let mut still_good = true;

    for c in line.chars() {
        still_good = match c {
            'b' => still_good && last != 'a',
            'd' => still_good && last != 'c',
            'q' => still_good && last != 'p',
            'y' => still_good && last != 'x',
            _ => still_good,
        };

        last = c;
    }
    return still_good;
}

fn check_repeated2(line: &str) -> bool {
    let mut last: char = ' ';
    let mut two_ago: char = ' ';

    for c in line.chars() {
        if c == two_ago {
            return true;
        }

        two_ago = last;
        last = c;
    }
    return false;
}

fn check_pairs(line: &str) -> bool {
    let mut last: char = ' ';
    let mut two_ago: char = ' ';

    let mut pairs: HashSet<String> = HashSet::new();

    for c in line.chars() {
        if c == last && last == two_ago {
            two_ago = ' '; // We want to allow aaaa to match two sets of aa
            continue;
        }

        let new_pair = last.to_string() + &c.to_string();

        let is_new = pairs.insert(new_pair);
        if !is_new {
            return true;
        }

        two_ago = last;
        last = c;
    }
    return false;
}
