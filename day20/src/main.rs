use prime_factorization::Factorization;
use std::collections::HashSet;

fn main() {
    let mut i = 1;
    let mut res = _presents_at2(i);
    while res < 36_000_000 {
        i += 1;
        res = _presents_at2(i);
    }
    println!("House {} got {} presents.", i, res);
}

fn _presents_at(house_number: u32) -> u32 {
    let factorization = Factorization::<u32>::run(house_number);

    let mut elves: HashSet<u32> = HashSet::new();
    elves.insert(1);

    for i in 0..factorization.factors.len() {
        let old_elves: HashSet<_> = HashSet::from_iter(elves.iter().map(|x| *x));
        for e in old_elves {
            elves.insert(factorization.factors[i] * e);
        }
    }

    let mut presents = 0;

    for elf in &elves {
        presents += 10*elf;
    }
    presents
}

fn _presents_at2(house_number: u32) -> u32 {
    let factorization = Factorization::<u32>::run(house_number);

    let mut elves: HashSet<u32> = HashSet::new();
    elves.insert(1);

    for i in 0..factorization.factors.len() {
        let old_elves: HashSet<_> = HashSet::from_iter(elves.iter().map(|x| *x));
        for e in old_elves {
            elves.insert(factorization.factors[i] * e);
        }
    }

    let mut presents = 0;

    for elf in &elves {
        if house_number/elf <= 50 {
            presents += 11*elf;
        }
    }
    presents
}