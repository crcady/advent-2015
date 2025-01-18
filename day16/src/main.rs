use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let mut aunts: Vec<Reading> = Vec::new();

    for line in io::stdin().lock().lines() {
        aunts.push(Reading::from_line(&line.unwrap()));
    }

    let present = Reading::from_present();

    for aunt in &aunts {
        if present.agrees_with(&aunt) {
            println!("Aunt Sue {} is a good first answer", aunt.index.unwrap());
        }

        if present.agrees_with2(&aunt) {
            println!("Aunt Sue {} is a good second answer", aunt.index.unwrap());
        }
    }
}

struct Reading {
    children: Option<u32>,
    cats: Option<u32>,
    samoyeds: Option<u32>,
    pomeranians: Option<u32>,
    akitas: Option<u32>,
    vizslas: Option<u32>,
    goldfish: Option<u32>,
    trees: Option<u32>,
    cars: Option<u32>,
    perfumes: Option<u32>,
    index: Option<u32>,
}

impl Reading {
    fn from_line(line: &str) -> Self {
        // Line has the following format:
        // Sue 1: goldfish: 9, cars: 0, samoyeds: 9

        let mut splits = line.split(" ").map(|x| x.trim_end_matches(&[',', ':']));

        splits.next(); // Consumes Sue

        let index: u32 = splits.next().unwrap().parse().unwrap();

        let mut vals: HashMap<&str, u32> = HashMap::new();
        let mut val_name: Option<&str> = None;

        for s in splits {
            match val_name {
                Some(name) => {
                    vals.insert(name, s.parse().unwrap());
                    val_name = None;
                }
                _ => val_name = Some(s),
            };
        }

        Self {
            children: vals.remove("children"),
            cats: vals.remove("cats"),
            samoyeds: vals.remove("samoyeds"),
            pomeranians: vals.remove("pomeranians"),
            akitas: vals.remove("akitas"),
            vizslas: vals.remove("vizslas"),
            goldfish: vals.remove("goldfish"),
            trees: vals.remove("trees"),
            cars: vals.remove("cars"),
            perfumes: vals.remove("perfumes"),
            index: Some(index),
        }
    }

    fn from_present() -> Self {
        Self {
            children: Some(3),
            cats: Some(7),
            samoyeds: Some(2),
            pomeranians: Some(3),
            akitas: Some(0),
            vizslas: Some(0),
            goldfish: Some(5),
            trees: Some(3),
            cars: Some(2),
            perfumes: Some(1),
            index: None,
        }
    }

    fn agrees_with(&self, other: &Self) -> bool {
        let compare_ish = |a: Option<u32>, b: Option<u32>| {
            if let Some(a_val) = a {
                if let Some(b_val) = b {
                    return a_val == b_val;
                }
            }
            true
        };
        compare_ish(self.children, other.children)
            && compare_ish(self.cats, other.cats)
            && compare_ish(self.samoyeds, other.samoyeds)
            && compare_ish(self.pomeranians, other.pomeranians)
            && compare_ish(self.akitas, other.akitas)
            && compare_ish(self.vizslas, other.vizslas)
            && compare_ish(self.goldfish, other.goldfish)
            && compare_ish(self.trees, other.trees)
            && compare_ish(self.cars, other.cars)
            && compare_ish(self.perfumes, other.perfumes)
    }

    fn agrees_with2(&self, other: &Self) -> bool {
        let compare_ish = |a: Option<u32>, b: Option<u32>| {
            if let Some(a_val) = a {
                if let Some(b_val) = b {
                    return a_val == b_val;
                }
            }
            true
        };

        let less_ish = |a: Option<u32>, b: Option<u32>| {
            if let Some(a_val) = a {
                if let Some(b_val) = b {
                    return a_val < b_val;
                }
            }
            true
        };

        let more_ish = |a: Option<u32>, b: Option<u32>| {
            if let Some(a_val) = a {
                if let Some(b_val) = b {
                    return a_val > b_val;
                }
            }
            true
        };
        
        compare_ish(self.children, other.children)
            && less_ish(self.cats, other.cats)
            && compare_ish(self.samoyeds, other.samoyeds)
            && more_ish(self.pomeranians, other.pomeranians)
            && compare_ish(self.akitas, other.akitas)
            && compare_ish(self.vizslas, other.vizslas)
            && more_ish(self.goldfish, other.goldfish)
            && less_ish(self.trees, other.trees)
            && compare_ish(self.cars, other.cars)
            && compare_ish(self.perfumes, other.perfumes)
    }
}
