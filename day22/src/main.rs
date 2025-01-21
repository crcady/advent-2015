fn main() {
    let wizard = Wizard::new();
    let boss = Boss::new();
    let gs = GameState::new(wizard, boss);

    let mut states = vec![TurnResult::KeepGoing(gs)];

    loop {
        states.sort_by(|a: &TurnResult, b: &TurnResult| to_key(a).cmp(&to_key(b)));
        //dbg!(&states);
        let current = states.remove(0);

        //dbg!(&current);
        match current {
            TurnResult::Lose => unimplemented!(),
            TurnResult::Win(mana_used, turns) => {
                println!("Won in {} turns using {} mana", turns, mana_used);
                break;
            }
            TurnResult::KeepGoing(gs) => {
                if gs.turns % 2 == 0 {
                    let next_turns = gs.player_turn(false);
                    for t in next_turns.into_iter() {
                        match t {
                            TurnResult::Lose => (),
                            _ => states.push(t),
                        }
                    }
                } else {
                    let t = gs.boss_turn();
                    match t {
                        TurnResult::Lose => (),
                        _ => states.push(t),
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
struct Wizard {
    // Intrinsic qualities
    hp: u32,
    mana: u32,

    // Only modified during gameplay
    armor: u32,
    shield_turns: u32,
    poison_turns: u32,
    recharge_turns: u32,
}

impl Wizard {
    fn new() -> Self {
        Self {
            hp: 50,
            mana: 500,
            armor: 0,
            shield_turns: 0,
            poison_turns: 0,
            recharge_turns: 0,
        }
    }

    fn apply(&self, hard_mode: bool) -> Self {
        let armor: u32 = match self.shield_turns > 0 {
            true => 7,
            false => 0,
        };

        let shield_turns = match self.shield_turns > 0 {
            true => self.shield_turns - 1,
            false => 0,
        };

        let poison_turns = match self.poison_turns > 0 {
            true => self.poison_turns - 1,
            false => 0,
        };

        let mana = match self.recharge_turns > 0 {
            true => self.mana + 101,
            false => self.mana,
        };

        let recharge_turns = match self.recharge_turns > 0 {
            true => self.recharge_turns - 1,
            false => 0,
        };

        let hp = match hard_mode {
            true => self.hp - 1,
            false => self.hp,
        };

        Self {
            hp,
            mana,
            armor,
            shield_turns,
            poison_turns,
            recharge_turns,
            ..*self
        }
    }

    fn magic_missile(&self) -> Self {
        Self {
            mana: self.mana - 53,
            ..*self
        }
    }

    fn drain(&self) -> Self {
        Self {
            mana: self.mana - 73,
            hp: self.hp + 2,
            ..*self
        }
    }

    fn shield(&self) -> Self {
        Self {
            mana: self.mana - 113,
            shield_turns: 6,
            ..*self
        }
    }

    fn poison(&self) -> Self {
        Self {
            mana: self.mana - 173,
            poison_turns: 6,
            ..*self
        }
    }

    fn recharge(&self) -> Self {
        Self {
            mana: self.mana - 229,
            recharge_turns: 5,
            ..*self
        }
    }

    fn hit_for(&self, damage: u32) -> Self {
        Self {
            hp: self.hp - damage,
            ..*self
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct Boss {
    hp: u32,
    attack: u32,
}

impl Boss {
    fn new() -> Self {
        Self { hp: 55, attack: 8 }
    }

    fn poison(&self) -> Self {
        Self {
            hp: self.hp - 3,
            ..*self
        }
    }

    fn magic_missile(&self) -> Self {
        Self {
            hp: self.hp - 4,
            ..*self
        }
    }

    fn drain(&self) -> Self {
        Self {
            hp: self.hp - 2,
            ..*self
        }
    }
}
#[derive(Debug)]
struct GameState {
    wizard: Wizard,
    boss: Boss,
    mana_used: u32,
    turns: u32,
}

impl GameState {
    fn new(wizard: Wizard, boss: Boss) -> Self {
        Self {
            wizard,
            boss,
            mana_used: 0,
            turns: 0,
        }
    }

    fn player_turn(&self, hard_mode: bool) -> Vec<TurnResult> {
        let mut boss = self.boss;

        if hard_mode && self.wizard.hp == 1 {
            return vec![TurnResult::Lose];
        }

        // Need to handle poison first
        if self.wizard.poison_turns > 0 {
            if self.boss.hp <= 3 {
                return vec![TurnResult::Win(self.mana_used, self.turns)];
            }
            boss = boss.poison();
        }

        // Apply start-of-turn conditions
        let wizard = self.wizard.apply(hard_mode);

        // See if we can cast a spell
        if wizard.mana < 53 {
            return vec![TurnResult::Lose];
        }

        let mut outcomes: Vec<TurnResult> = Vec::new();
        if boss.hp <= 4 {
            outcomes.push(TurnResult::Win(self.mana_used + 53, self.turns));
        } else {
            outcomes.push(TurnResult::KeepGoing(GameState {
                wizard: wizard.magic_missile(),
                boss: boss.magic_missile(),
                mana_used: self.mana_used + 53,
                turns: self.turns + 1,
            }));
        }

        if wizard.mana < 73 {
            return outcomes;
        }

        if boss.hp <= 2 {
            outcomes.push(TurnResult::Win(self.mana_used + 73, self.turns));
        } else {
            outcomes.push(TurnResult::KeepGoing(GameState {
                wizard: wizard.drain(),
                boss: boss.drain(),
                mana_used: self.mana_used + 73,
                turns: self.turns + 1,
            }));
        }

        if wizard.mana < 113 {
            return outcomes;
        }

        if wizard.shield_turns == 0 {
            outcomes.push(TurnResult::KeepGoing(GameState {
                wizard: wizard.shield(),
                boss: boss.clone(),
                mana_used: self.mana_used + 113,
                turns: self.turns + 1,
            }));
        }

        if wizard.mana < 173 {
            return outcomes;
        }

        if wizard.poison_turns == 0 {
            outcomes.push(TurnResult::KeepGoing(GameState {
                wizard: wizard.poison(),
                boss: boss.clone(),
                mana_used: self.mana_used + 173,
                turns: self.turns + 1,
            }));
        }

        if wizard.mana < 229 {
            return outcomes;
        }

        if wizard.recharge_turns == 0 {
            outcomes.push(TurnResult::KeepGoing(GameState {
                wizard: wizard.recharge(),
                boss: boss.clone(),
                mana_used: self.mana_used + 229,
                turns: self.turns + 1,
            }));
        }

        outcomes
    }

    fn boss_turn(&self) -> TurnResult {
        // The boss only dies or attacks, so there's only one TurnResult
        let mut boss = self.boss;

        // Need to handle poison first
        if self.wizard.poison_turns > 0 {
            if self.boss.hp <= 3 {
                return TurnResult::Win(self.mana_used, self.turns);
            }
            boss = boss.poison();
        }

        // Apply start-of-turn conditions
        let wizard = self.wizard.apply(false);

        let damage_done = match boss.attack > wizard.armor {
            true => boss.attack - wizard.armor,
            false => 1,
        };

        if damage_done >= wizard.hp {
            return TurnResult::Lose;
        }

        let wizard = wizard.hit_for(damage_done);

        return TurnResult::KeepGoing(GameState {
            wizard,
            boss,
            mana_used: self.mana_used,
            turns: self.turns + 1,
        });
    }
}

#[derive(Debug)]
enum TurnResult {
    Lose,
    Win(u32, u32), // Mana and turns
    KeepGoing(GameState),
}

fn to_key(tr: &TurnResult) -> u32 {
    match tr {
        TurnResult::Lose => unimplemented!(),
        TurnResult::Win(mana_used, _) => *mana_used,
        TurnResult::KeepGoing(game_state) => game_state.mana_used,
    }
}
