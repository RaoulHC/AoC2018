use std::fs;
use std::collections::HashMap;

fn reduce(input: &mut Vec<char>) -> u32 {
    let mut counter = 0;
    let mut modified = false;

    loop {
        let letter_2 = match input.get(counter+1) {
            Some(letter) => letter,
            None => {
                if modified == true {
                    modified = false;
                    counter = 0;
                    continue;
                } else {
                    break;
                }
            }
        };
        let letter_1 = input.get(counter).unwrap();

        if letter_1.to_lowercase().to_string()
            == letter_2.to_lowercase().to_string() {
            if (letter_1.is_lowercase() && letter_2.is_uppercase())
                || (letter_1.is_uppercase() && letter_2.is_lowercase()) {
                input.remove(counter);
                input.remove(counter);
                modified = true;
                }
        }

        counter += 1;
    }
    return input.len() as u32;
}

// much better solution from AoC megathread, this stops having lots of loops and
// deals with reactions as they come, as well as not needing to reallocate the
// vector constantly.
fn reduce_2(input: impl Iterator<Item = char>) -> usize {
    let mut vec = Vec::new();
    for c in input {
        match vec.last() {
            None => vec.push(c),
            Some(d) => {
                if c.to_lowercase().to_string() == d.to_lowercase().to_string()
                && &c != d {
                    vec.pop();
                } else {
                    vec.push(c)
                }
            }
        }
    }
    vec.len()
}

fn main() {
    let mut input = fs::read_to_string("input.txt").unwrap();
    input.truncate(input.trim_end().len());
    let input: Vec<char> = input.chars().collect();
    println!("input result length: {}", reduce(&mut input.clone()));

    let mut letter_lengths: HashMap<char, usize> = HashMap::new();
    for letter in "abcdefghijklmnopqrstuvwxyz".chars() {
        let mut new_input: Vec<char> = input.clone();
        new_input.retain(
            |&x| x.to_lowercase().to_string() != letter.to_string());
        let length = reduce_2(new_input.into_iter());
        println!("{}", length);
        letter_lengths.insert(letter, length);
    }
    println!("{:?}", letter_lengths);
}
