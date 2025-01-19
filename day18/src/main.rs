use std::io::{self, BufRead};

fn main() {
    let mut lg = LightGrid::new(100);
    let mut lg2 = LightGrid::new_stuck(100);

    let mut x: i32 = 0;
    for line in io::stdin().lock().lines() {
        let mut y: i32 = 0;
        for c in line.unwrap().chars() {
            match c {
                '.' => (),
                '#' => {
                    lg.set(Point(x, y));
                    lg2.set(Point(x, y));
                }
                _ => unimplemented!(),
            }
            y += 1;
        }
        x += 1;
    }

    println!("Started with {} lights on", lg.count());
    for _ in 0..100 {
        lg.step();
        lg2.step();
    }
    println!("After 100 steps, {} lights are on", lg.count());
    println!("If the corners are stuck, {} lights are on", lg2.count());
}

struct LightGrid {
    size: usize,
    values: Vec<Vec<bool>>,
    get_fn: fn(&LightGrid, &Point) -> bool,
}

impl LightGrid {
    fn new(size: usize) -> Self {
        Self {
            size,
            values: vec![vec![false; size]; size],
            get_fn: LightGrid::get,
        }
    }

    fn new_stuck(size: usize) -> Self {
        Self {
            size,
            values: vec![vec![false; size]; size],
            get_fn: LightGrid::get2,
        }
    }

    fn set(&mut self, p: Point) {
        let Point(x, y) = p;
        self.values[x as usize][y as usize] = true;
    }

    fn count(&self) -> usize {
        let mut res: usize = 0;
        for x in 0..self.size {
            for y in 0..self.size {
                let p = Point(x as i32, y as i32);
                if (self.get_fn)(self, &p) {
                    res += 1;
                }
            }
        }
        res
    }

    fn get(&self, p: &Point) -> bool {
        let Point(x, y) = p;
        if *x >= 0
            && *x < self.size.try_into().unwrap()
            && *y >= 0
            && *y < self.size.try_into().unwrap()
        {
            return self.values[*x as usize][*y as usize];
        }
        false
    }

    fn get2(&self, p: &Point) -> bool {
        // Responds as if the corner lights are stuck
        let Point(x, y) = p;
        let size: i32 = self.size.try_into().unwrap();

        if (*x == 0 || *x == size - 1) && (*y == 0 || *y == size - 1) {
            return true;
        }

        if *x >= 0 && *x < size && *y >= 0 && *y < size {
            return self.values[*x as usize][*y as usize];
        }
        false
    }

    fn step(&mut self) {
        let mut new_values: Vec<Vec<bool>> = vec![vec![false; self.size]; self.size];

        for x in 0..self.size {
            for y in 0..self.size {
                let p = Point(x.try_into().unwrap(), y.try_into().unwrap());
                new_values[x][y] = match (self.get_fn)(self, &p) {
                    true => self.step_on(&p),
                    false => self.step_off(&p),
                }
            }
        }
        self.values = new_values;
    }

    fn step_on(&self, p: &Point) -> bool {
        // Lights that are on stay on when exactly 2 or 3 neighbors are on, and turn off otherwise
        let mut neighbor_count = 0;
        for n in p.neighbors() {
            if (self.get_fn)(self, &n) {
                neighbor_count += 1;
            }
        }
        match neighbor_count {
            2 | 3 => true,
            _ => false,
        }
    }

    fn step_off(&self, p: &Point) -> bool {
        // Lights that are off turn on if exactly 3 neighbors are on
        let mut neighbor_count = 0;
        for n in p.neighbors() {
            if (self.get_fn)(self, &n) {
                neighbor_count += 1;
            }
        }
        match neighbor_count {
            3 => true,
            _ => false,
        }
    }
}

struct Point(i32, i32);

impl Point {
    fn neighbors(&self) -> Vec<Point> {
        let Point(x, y) = self;
        vec![
            Point(x - 1, y - 1),
            Point(x - 1, *y),
            Point(x - 1, y + 1),
            Point(*x, y - 1),
            Point(*x, y + 1),
            Point(x + 1, *y - 1),
            Point(x + 1, *y),
            Point(x + 1, y + 1),
        ]
    }
}
