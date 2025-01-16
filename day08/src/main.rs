use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let fname = match args.len() {
        1 => "example.txt",
        _ => &args[1],
    };

    let mut character_count = 0;
    let mut memory_count = 0;
    let mut expanded_count = 0;

    let file = File::open(fname).expect("Couldn't open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let chars: Vec<char> = line.expect("Didn't get a line").chars().collect();

        let num_chars = chars.len();
        let mut mems = 0;
        let mut expanded_chars = 2;

        let mut i: usize = 1;

        while i < num_chars - 1 {
            mems += 1;

            i = match chars[i] {
                '\\' => match chars[i + 1] {
                    'x' => i + 4,
                    _ => i + 2,
                },
                _ => i + 1,
            }
        }

        for c in chars {
            expanded_chars = match c {
                '\"' => expanded_chars + 2,
                '\\' => expanded_chars + 2,
                _ => expanded_chars + 1,
            }
        }

        character_count += num_chars;
        memory_count += mems;
        expanded_count += expanded_chars;
    }

    println!(
        "Found {} chars of string code and {} characters in memory, for a difference of {}",
        &character_count,
        &memory_count,
        character_count - memory_count
    );

    println!(
        "Still have {} chars of string code, but expanded it to {} chars, for a difference of {}",
        &character_count,
        &expanded_count,
        expanded_count - character_count
    );
}
