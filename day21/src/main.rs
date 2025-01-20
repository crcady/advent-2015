fn main() {
    let player = PlayerStats {
        hp: 100,
        damage: 0,
        armor: 0,
        cost: 0,
    };
    let boss = PlayerStats {
        hp: 103,
        damage: 9,
        armor: 2,
        cost: 0,
    };

    let weapons: Vec<Item> = vec![
        Item {
            cost: 8,
            damage_bonus: 4,
            armor_bonus: 0,
        },
        Item {
            cost: 10,
            damage_bonus: 5,
            armor_bonus: 0,
        },
        Item {
            cost: 25,
            damage_bonus: 6,
            armor_bonus: 0,
        },
        Item {
            cost: 40,
            damage_bonus: 7,
            armor_bonus: 0,
        },
        Item {
            cost: 74,
            damage_bonus: 8,
            armor_bonus: 0,
        },
    ];

    let armors: Vec<Item> = vec![
        Item {
            cost: 0,
            damage_bonus: 0,
            armor_bonus: 0,
        },
        Item {
            cost: 13,
            damage_bonus: 0,
            armor_bonus: 1,
        },
        Item {
            cost: 31,
            damage_bonus: 0,
            armor_bonus: 2,
        },
        Item {
            cost: 53,
            damage_bonus: 0,
            armor_bonus: 3,
        },
        Item {
            cost: 75,
            damage_bonus: 0,
            armor_bonus: 4,
        },
        Item {
            cost: 102,
            damage_bonus: 0,
            armor_bonus: 5,
        },
    ];

    let rings: Vec<Item> = vec![
        Item {
            cost: 0,
            damage_bonus: 0,
            armor_bonus: 0,
        },
        Item {
            cost: 0,
            damage_bonus: 0,
            armor_bonus: 0,
        },
        Item {
            cost: 25,
            damage_bonus: 1,
            armor_bonus: 0,
        },
        Item {
            cost: 50,
            damage_bonus: 2,
            armor_bonus: 0,
        },
        Item {
            cost: 100,
            damage_bonus: 3,
            armor_bonus: 0,
        },
        Item {
            cost: 20,
            damage_bonus: 0,
            armor_bonus: 1,
        },
        Item {
            cost: 40,
            damage_bonus: 0,
            armor_bonus: 2,
        },
        Item {
            cost: 80,
            damage_bonus: 0,
            armor_bonus: 3,
        },
    ];

    let mut combinations: Vec<PlayerStats> = Vec::new();
    for wpn in &weapons {
        for arm in &armors {
            for i in 0..rings.len() {
                for j in 0..rings.len() {
                    if i == j {
                        continue;
                    }
                    let ring1 = &rings[i];
                    let ring2 = &rings[j];
                    combinations.push(
                        player
                            .add_item(wpn)
                            .add_item(arm)
                            .add_item(ring1)
                            .add_item(ring2),
                    );
                }
            }
        }
    }
    println!("Founds {} combos", combinations.len());
    combinations.sort_by(|x, y| x.cost.cmp(&y.cost));
    for config in &combinations {
        if config.wins_against(&boss) {
            println!("Won by spending {} gold", config.cost);
            break;
        }
    }

    combinations.reverse();

    for config in &combinations {
        if !config.wins_against(&boss) {
            println!("Man, I spend {} gold and still lost!", config.cost);
            break;
        }
    }
}

#[derive(Debug)]
struct PlayerStats {
    hp: u32,
    damage: u32,
    armor: u32,
    cost: u32,
}

struct Item {
    cost: u32,
    damage_bonus: u32,
    armor_bonus: u32,
}

impl PlayerStats {
    fn wins_against(&self, other: &PlayerStats) -> bool {
        // Returns true of the player wins against other
        // Self is the attacker

        let mut self_hp_repaining = self.hp;
        let mut other_hp_remaining = other.hp;
        let self_damage = match self.damage > other.armor {
            true => self.damage - other.armor,
            false => 1,
        };

        let other_damage = match other.damage > self.armor {
            true => other.damage - self.armor,
            false => 1,
        };

        loop {
            if other_hp_remaining > self_damage {
                other_hp_remaining -= self_damage;
            } else {
                return true;
            }

            if self_hp_repaining > other_damage {
                self_hp_repaining -= other_damage;
            } else {
                return false;
            }
        }
    }

    fn add_item(&self, item: &Item) -> Self {
        Self {
            damage: self.damage + item.damage_bonus,
            armor: self.armor + item.armor_bonus,
            cost: self.cost + item.cost,
            ..*self
        }
    }
}
