use std::fs;
extern crate regex;
use regex::Regex;

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Claim {
    id: u32,
    pos: Coord,
    size: Coord,
}

// Something to take in a string and give me a claim structure
fn parse_claim(input_claim: &str) -> Claim {
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let cap = re.captures(input_claim).unwrap();
    let pos = Coord {
        x: cap[2].parse().unwrap(),
        y: cap[3].parse().unwrap(),
    };
    let size = Coord {
        x: cap[4].parse().unwrap(),
        y: cap[5].parse().unwrap(),
    };
    Claim {
        id: cap[1].parse().unwrap(),
        pos: pos,
        size: size,
    }
}

// Something to borrow a 2d array and update positions
fn update_array(claim: Claim, array: &mut Vec<Vec<usize>>) {
    for i in claim.pos.x..(claim.pos.x+claim.size.x) {
        for j in claim.pos.y..(claim.pos.y+claim.size.y) {
            array[i][j] += 1;
        }
    }
}

fn count_double_claims(vec: &Vec<Vec<usize>>) -> usize {
    let mut count = 0;
    for i in vec.iter() {
        for j in i.iter() {
            if *j > 1 {
                count += 1;
            }
        }
    }
    count
}

fn main () {
    // check that parsing works
    let claim = parse_claim("#1 @ 1,2: 4x4");
    println!("Claim: {:?}", claim);

    let f = fs::read_to_string("input.txt").unwrap();
    let mut vec : Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];
    for cl in f.lines() {
        update_array(parse_claim(cl), &mut vec);
    }
    println!("Number of double claims: {}", count_double_claims(&vec))
}
