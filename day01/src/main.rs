use std::io;

fn main() {
    println!("Enter some parentheses!");

    let mut user_input = String::new();

    io::stdin().read_line(&mut user_input).expect("Failed to read line");
    let user_input = user_input.trim();

    let floor = line_to_floor(user_input);
    let basement_floor = find_basement(user_input);

    println!("Ended up on floor {}", floor);
    println!("Found the basement after {} characters", basement_floor)
}

fn line_to_floor(line: &str) -> i32 {
	let mut floor = 0;
	for c in line.chars() {
		match c {
			'(' => floor +=  1,
			')' => floor -=  1,
			_ => println!("Unexpected character: {}", c),
		}
	}
	floor
}

fn find_basement(line: &str) -> i32 {
	let mut floor = 0;
	let mut index = 0;
	
	for c in line.chars() {
		if floor == -1 {
			return index
		}
		index += 1;
		match c {
			'(' => floor += 1,
			')' => floor -= 1,
			_ => println!("Unexpected character: {}", c),
		}
	}
	if floor == -1 {
		return index
	}
	-1
}
