fn main() {
    let mut p = Page::new(2978, 3083);
    while p.step() {}
    println!("Found the value: {}", p.current);
}

struct Page {
    row: u64,
    col: u64,
    current: u64,
    target_row: u64,
    target_col: u64,
}

impl Page {
    fn new(target_row: u64, target_col: u64) -> Self {
        Self {
            row: 1,
            col: 1,
            current: 20151125,
            target_row,
            target_col,
        }
    }
    fn step(&mut self) -> bool {
        self.current = (self.current * 252533) % 33554393;
        if self.row == 1 {
            println!("Finished diagonal {}/{}", self.col, self.target_col + self.target_row - 2);
            self.row = self.col + 1;
            self.col = 1;
        } else {
            self.row -= 1;
            self.col += 1;
        }
        self.row != self.target_row || self.col != self.target_col
    }
}