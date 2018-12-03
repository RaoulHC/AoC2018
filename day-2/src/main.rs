use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn process(id : &str) -> (bool, bool) {
    let mut two_count = false;
    let mut three_count = false;
    let mut map : HashMap<char, i32> = HashMap::new();
    for letter in id.chars() {
        let refr = map.entry(letter).or_insert(0);
        *refr += 1;
    };
    for val in map.values() {
        if *val == 2 {
            two_count = true;
        } else if *val == 3 {
            three_count = true;
        }
    }
    (two_count, three_count)
}

fn main() {
    // open file
    let mut f = File::open("src/data.txt")
        .expect("File not found");

    // read in data
    let mut data = String::new();
    f.read_to_string(&mut data)
        .expect("Something went wrong reading the file.");

    let mut two_count = 0;
    let mut three_count = 0;
    for line in data.lines() {
        let (twos, threes) = process(line);
        if twos {two_count += 1;}
        if threes {three_count += 1;}
    }
    let checksum = two_count * three_count;
    println!("two count: {}", two_count);
    println!("thee count: {}", three_count);
    println!("Checksum: {}", checksum);
}
