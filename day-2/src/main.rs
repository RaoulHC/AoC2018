use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

// Counts letters and returns whether there is a set of two or three
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

fn find_ids(data : &str) -> (&str, &str) {
    // want to compare all id's to find the first two that differ by one
    let mut line1;
    let mut rest = data;
    for line in data.lines() {
        line1 = line;
        rest = cut_first_line(rest);
        for line2 in rest.lines() {
            if compare(line1, line2) == true {
                return (line1, line2);
            }
        }
    }
    ("", "")
}

fn cut_first_line(s : &str) -> &str {
    let indice = s.find('\n').unwrap() + 1;
    &s[indice..]
}

fn compare(s : &str, t : &str) -> bool {
    let mut count : i32 = 0;
    let s_bytes = s.as_bytes();
    let t_bytes = t.as_bytes();
    for (i, chr) in s_bytes.iter().enumerate() {
        if chr != &t_bytes[i] {
            count += 1;
            if count > 1 {
                return false;
            }
        }
    }
    if count == 1 {
        return true;
    } else {
        return false;
    }
}

fn find_common_letters(s : &str, t : &str) -> usize {
    for (i, x) in s.chars().enumerate() {
        if x != t.chars().nth(i).unwrap() {
            return i;
        }
    }
    return 0;
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

    println!("Finding almost matching id's");
    let (id1, id2) = find_ids(&data);
    println!("id1: {}\nid2: {}", id1, id2);
    let indice = find_common_letters(id1, id2);
    for (i, x) in id1.chars().enumerate() {
        if i == indice {
            continue;
        }
        print!("{}", x);
    }
    println!("");
}
