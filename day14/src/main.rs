use std::io::{self, BufRead};

fn main() {
    println!("Tell me about your reindeer in the format:\nComet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.\nctrl-d to end");

    let mut racers: Vec<Racer> = Vec::new();

    for line in io::stdin().lock().lines() {
        let stats = parse_line(&line.unwrap());
        let racer = Racer::new(&stats);
        racers.push(racer);
    }

    for _ in 0..2503 {
        for i in 0..racers.len() {
            racers[i] = racers[i].step();
        }

        let mut max_distance = 0;
        for r in &racers {
            if r.position > max_distance {
                max_distance = r.position;
            }
        }

        for i in 0..racers.len() {
            if racers[i].position == max_distance {
                racers[i] = racers[i].award();
            }
        }
    }

    let mut max_distance = 0;
    let mut max_points = 0;
    for r in &racers {
        if r.position > max_distance {
            max_distance = r.position;
        }
        if r.points > max_points {
            max_points = r.points;
        }
    }

    println!("The fastest reindeer went {} km", max_distance);
    println!("The most awarded reindeer had {} points", max_points);
}

#[derive(Debug, Clone, Copy)]
struct Racer {
    position: u32,
    resting: bool,
    time_left: u32,
    points: u32,
    stats: StatBlock,
}

impl Racer {
    fn new(stats: &StatBlock) -> Self {
        Self {
            position: 0,
            resting: false,
            time_left: stats.run_time,
            points: 0,
            stats: stats.clone(),
        }
    }

    fn step(self) -> Self {
        let position = match self.resting {
            false => self.position + self.stats.speed,
            true => self.position,
        };

        let resting = match self.time_left {
            1 => !self.resting,
            _ => self.resting,
        };

        let time_left = match self.time_left {
            1 => match resting {
                true => self.stats.rest_time,
                false => self.stats.run_time,
            },
            _ => self.time_left - 1,
        };

        Self {
            position,
            resting,
            time_left,
            points: self.points,
            stats: self.stats.clone(),
        }
    }

    fn award(self) -> Self {
        Self {
            position: self.position,
            resting: self.resting,
            time_left: self.time_left,
            points: self.points + 1,
            stats: self.stats.clone(),
        }
    }
}
#[derive(Debug, Clone, Copy)]
struct StatBlock {
    speed: u32,
    run_time: u32,
    rest_time: u32,
}

fn parse_line(line: &str) -> StatBlock {
    // The line is in the format:
    // Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
    //  [0]  [1] [2] [3] [4] [5] [6]   [7]   [8]  [9] [10] [11] [12] [13] [14]

    let splits: Vec<&str> = line.split(" ").collect();

    StatBlock {
        speed: splits[3].parse().unwrap(),
        run_time: splits[6].parse().unwrap(),
        rest_time: splits[13].parse().unwrap(),
    }
}
