use std::io::{self, BufRead};

fn main() {
    let mut ingredients: Vec<IngredientProperties> = Vec::new();

    for line in io::stdin().lock().lines() {
        ingredients.push(IngredientProperties::from_line(&line.unwrap()));
    }

    let recipes = generate_vectors(100, ingredients.len());

    let mut best_score = 0u32;
    let mut constrained_score = 0u32;

    for r in &recipes {
        let s = score(&ingredients, &r, None);
        let s2 = score(&ingredients, &r, Some(500));
        if s > best_score {
            best_score = s;
        }
        if s2 > constrained_score {
            constrained_score = s2;
        }
    }

    println!("The best cookie scored {}", best_score);
    println!(
        "But if you count calories, you only get {}",
        constrained_score
    );
}

#[derive(Debug)]
struct IngredientProperties {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl IngredientProperties {
    fn from_line(line: &str) -> Self {
        // The line has the format:
        // Sprinkles: capacity 5, durability -1, flavor 0, texture 0, calories 5
        //    [0]       [1]   [2]    [3]     [4]   [5] [6]  [7]    [8]  [9]   [10]

        let splits: Vec<&str> = line.split(' ').map(|x| x.trim_end_matches(',')).collect();

        Self {
            capacity: splits[2].parse().unwrap(),
            durability: splits[4].parse().unwrap(),
            flavor: splits[6].parse().unwrap(),
            texture: splits[8].parse().unwrap(),
            calories: splits[10].parse().unwrap(),
        }
    }
}

fn generate_vectors(total: u32, length: usize) -> Vec<Vec<u32>> {
    let mut res: Vec<Vec<u32>> = Vec::new();

    if length == 1 {
        res.push(vec![total]);
    } else {
        for i in 0..=total {
            for v in generate_vectors(total - i, length - 1) {
                res.push([vec![i], v].concat());
            }
        }
    }
    res
}

fn score(
    ingredients: &Vec<IngredientProperties>,
    recipe: &Vec<u32>,
    calorie_target: Option<u32>,
) -> u32 {
    assert_eq!(
        ingredients.len(),
        recipe.len(),
        "Have {} ingredients but recipe has {} entries",
        ingredients.len(),
        recipe.len()
    );

    let mut capacity_score = 0;
    let mut durability_score = 0;
    let mut flavor_score = 0;
    let mut texture_score = 0;
    let mut calories_score = 0;

    for i in 0..ingredients.len() {
        capacity_score += ingredients[i].capacity * recipe[i] as i32;
        durability_score += ingredients[i].durability * recipe[i] as i32;
        flavor_score += ingredients[i].flavor * recipe[i] as i32;
        texture_score += ingredients[i].texture * recipe[i] as i32;
        calories_score += ingredients[i].calories * recipe[i] as i32;
    }

    let capacity_score: u32 = match capacity_score < 0 {
        true => 0,
        false => capacity_score as u32,
    };

    let durability_score: u32 = match durability_score < 0 {
        true => 0,
        false => durability_score as u32,
    };

    let flavor_score: u32 = match flavor_score < 0 {
        true => 0,
        false => flavor_score as u32,
    };

    let texture_score: u32 = match texture_score < 0 {
        true => 0,
        false => texture_score as u32,
    };

    let calories_score: u32 = match calories_score < 0 {
        true => 0,
        false => calories_score as u32,
    };

    match calorie_target {
        Some(tgt) => match calories_score == tgt {
            true => capacity_score * durability_score * flavor_score * texture_score,
            false => 0,
        },
        _ => capacity_score * durability_score * flavor_score * texture_score,
    }
}
