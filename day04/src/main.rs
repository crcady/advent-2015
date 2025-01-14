use std::io;

fn main() {
    let mut user_input = String::new();
    println!("What's your secret key?");
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");
    let user_input = user_input.trim();

    let mut success = false;
    let mut num = -1;

    while !success {
        num += 1;
        let md5_input = user_input.to_owned() + &num.to_string();
        let digest = format!("{:x}", md5::compute(md5_input.clone()))
            .chars()
            .collect::<Vec<char>>();

        success = true;
        for i in 0..6 {
            success = match digest[i] {
                '0' => success,
                _ => false,
            }
        }
    }

    println!("Mined a coin at {}", num);
}
