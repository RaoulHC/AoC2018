use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn count(data : String) -> i32 {
    // count up things
    let mut count : i32 = 0;
    let mut temp : i32;
    for line in data.lines() {
        temp = match line.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Couldn't read line!");
                break;
            },
        };
        count += temp;
    };
    count
}

fn find_double(data : String) -> i32 {
    // find the first double repeating through the whole list.
    let mut map = HashMap::new();
    let mut line_number : i32;
    let mut count : i32 = 0;
    let mut found : bool = false;
    loop {
        if found {break}
        for line in data.lines() {
            line_number = match line.parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Couldn't read line!");
                    break;
                }
            };
            count += line_number;
            if !map.contains_key(&count) {
                map.insert(count, ());
            } else {
                found = true;
                break;
            }
        };
    };
    count
}

fn main() {
    // open file
    let mut f = File::open("src/data.txt")
        .expect("File not found");

    // read in data
    let mut data = String::new();
    f.read_to_string(&mut data)
        .expect("Something went wrong reading the file.");

    let lines = data;

    println!("count: {}", count(lines.clone()));
    println!("first double: {}", find_double(lines.clone()));
}
