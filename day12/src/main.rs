use serde_json::Value;
use std::{env, fs::File};

fn main() {
    let args: Vec<String> = env::args().collect();
    let fname = match args.len() {
        1 => "input.json",
        _ => &args[1],
    };

    let file = File::open(fname).expect("Failed to open file");
    let v: Value = serde_json::from_reader(file).expect("Couldn't parse the JSON");
    
    let total = count_nums(&v);

    let total2 = count_no_red(&v);

    println!("Total was: {}", total);
    println!("Total without red: {}", total2);
}

fn count_nums(v: &Value) -> i64 {
    match v {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(number) => number.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(vec) => vec.into_iter().map(|val| count_nums(val)).sum(),
        Value::Object(map) => map.clone().into_values().map(|val| count_nums(&val)).sum(),
    }
}

fn count_no_red(v: &Value) -> i64 {
    match v {
        Value::Null => 0,
        Value::Bool(_) => 0,
        Value::Number(number) => number.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(vec) => vec.into_iter().map(|val| count_no_red(val)).sum(),
        Value::Object(_) => handle_object(v),
    }
}

fn handle_object(o: &Value) -> i64 {
    let obj = o.as_object().unwrap(); // It's okay to panic if we somehow call this with another variant
    let mut partial = 0i64;
    for (k, v) in obj.into_iter() {
        if let Value::String(val) = v {
            if val == &String::from("red") {
                return 0;
            }
        }
        partial += count_no_red(v);
    }
    partial
}