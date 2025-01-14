use std::{char, collections::HashSet, hash::Hash, io};

fn main() {
    println!("Hello, world!");
    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let user_input = user_input.trim();

    let mut santa = build_santa();

    for c in user_input.chars() {
        santa.walk(c);
    }
    
    println!("Santa visitied {} unique houses", santa.been.len());

    let mut santa = build_santa();
    let mut robot = build_santa();

    for (i, c) in user_input.chars().enumerate() {
        if i % 2 == 0{
            santa.walk(c);
        } else {
            robot.walk(c);
        }
    }

    let all_locs = santa.been.union(&robot.been).collect::<Vec<&Location>>();

    println!("Santa and the robot together visitied {} unique houses", all_locs.len());

}

#[derive(PartialEq,Eq, Hash, Clone, Copy)]
struct Location {
    x: i32,
    y: i32,
}

struct Santa {
    loc: Location,
    been: HashSet<Location>,
}

fn build_santa() -> Santa {
    let initial_loc = Location {x: 0, y: 0};
    let mut hs: HashSet<Location> = HashSet::new();
    hs.insert(initial_loc);

    Santa { loc: initial_loc, been: hs }
}

impl Santa {
    fn walk(&mut self, c: char) {
        let x = self.loc.x;
        let y = self.loc.y;
        let new_loc = match c {
            '>' => Location { x: x+1, y: y },
            '<' => Location { x: x-1, y: y },
            'v' => Location { x: x, y: y+1 },
            '^' => Location { x: x, y: y-1},
            _ => Location {x: x, y: y}, // Do nothing for now
        };

        self.been.insert(new_loc);
        self.loc = new_loc;
    }
}