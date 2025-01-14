use std::{cmp::min, io, io::BufRead};
fn main() {
    println!("Give me some digits! LLxHHxWW");

    let mut ans1: u32 = 0;
    let mut ans2: u32 = 0;

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }

        let splits: Vec<&str> = line.split("x").collect();
        let length: u32 = splits[0].parse().unwrap();
        let width: u32 = splits[1].parse().unwrap();
        let height: u32 = splits[2].parse().unwrap();

        let pb = PresentBox {
            length,
            height,
            width,
        };

        let paper = pb.paper();
        let ribbon = pb.ribbon();
        ans1 += paper;
        ans2 += ribbon;
    }

    println!("We need {} units of wrapping paper", ans1);
    println!("We need {} units of ribbon", ans2);
}
struct PresentBox {
    length: u32,
    height: u32,
    width: u32,
}

impl PresentBox {
    fn paper(&self) -> u32 {
        let a = self.length * self.height;
        let b = self.length * self.width;
        let c = self.height * self.width;

        let smallest_side = min(min(a, b), c);

        2 * a + 2 * b + 2 * c + smallest_side
    }

    fn ribbon(&self) -> u32 {
        let a = 2 * (self.length + self.height);
        let b = 2 * (self.length + self.width);
        let c = 2 * (self.height + self.width);

        let smallest_perimeter = min(min(a, b), c);

        self.length * self.height * self.width + smallest_perimeter
    }
}
