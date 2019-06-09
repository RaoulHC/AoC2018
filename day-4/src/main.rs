use std::fs;
use std::collections::HashMap;

#[macro_use] extern crate lazy_static; // Static regular expressions
extern crate regex;
use regex::Regex;

type ID = u32;

#[derive(Debug, Clone)]
struct Entry {
    minute: u32,
    event: Event,
}

#[derive(Debug, Clone)]
enum Event {
    Sleeps,
    Wakes,
}

fn parse_entry(in_str: &str) -> Entry {
    lazy_static! {
        static ref re: Regex =
            Regex::new(r"\[\d{4}-\d{2}-\d{2} \d{2}:(\d{2})\] (.+)").unwrap();
    }
    let cap = re.captures(in_str).unwrap();
    Entry {
        minute: cap[1].parse().unwrap(),
        event: match &cap[2] {
            "falls asleep" => Event::Sleeps,
            "wakes up" => Event::Wakes,
            e => panic!("Unexpected log occured: {}", e),
        },
    }
}

fn main() {
    // Read and order lines
    let f = fs::read_to_string("input.txt").unwrap();
    let mut f : Vec<&str> = f.lines().collect();
    f.sort();

    // regex to split up vectors
    let guard_re: Regex =
        Regex::new(r"\[.+\] Guard #(\d+) begins shift").unwrap();

    // hashmap of guard sleep minutes
    // read in data and count number of minutes
    let mut guards_sleep: HashMap<ID, [u32; 60]> = HashMap::new();
    let mut sleep_mins: &mut [u32; 60] = &mut [0; 60];
    let mut counter: u32 = 0;
    let mut asleep: bool;
    for line in f {
        if guard_re.is_match(line) {
            let id = guard_re.captures(line).unwrap()[1].parse().unwrap();
            sleep_mins = guards_sleep.entry(id).or_insert([0; 60]);
            counter = 0;
            continue;
        }
        let entry = parse_entry(line);
        asleep = match entry.event {
            Event::Sleeps => true,
            Event::Wakes => false,
        };
        if !asleep {
            for i in counter as usize..entry.minute as usize {
                sleep_mins[i] += 1 as u32;
            }
        }
        counter = entry.minute;
    }

    // day 1 stuff, now get the number total number of minutes asleep
    let mut sleepy_time: HashMap<ID, u32> = HashMap::new();
    let mut sleep_counter: &mut u32;
    for (id, sleep) in guards_sleep.clone() {
        sleep_counter = sleepy_time.entry(id).or_insert(0);
        *sleep_counter = sleep.iter().sum();
    }
    let sleepy_id: u32 = *sleepy_time.iter()
        .max_by(|(_, x), (_, y)| x.cmp(y)).unwrap().0;
    println!("sleepiest ID: {:?}", sleepy_id);

    // and now find the minute it was most asleep in
    let sleepy_minute = guards_sleep
        .get(&sleepy_id).unwrap()
        .iter().enumerate()
        .max_by(|(_, x), (_, y)| x.cmp(y)).unwrap().0;
    println!("Sleepiest minute: {:?}", sleepy_minute);

    println!("ID x minute: {}", sleepy_id * sleepy_minute as u32);

    // day 2 stuff, need to look at the most asleep minute.
    let mut max_sleep_minute: u32 = 0;
    let mut max_sleep: u32 = 0;
    let mut sleepy_id: u32 = 0;
    for (id, sleep) in guards_sleep.clone() {
        let temp_sleep = sleep
            .iter().enumerate()
            .max_by(|(_, x), (_, y)| x.cmp(y)).unwrap();
        if temp_sleep.1 > &max_sleep {
            max_sleep = *temp_sleep.1;
            max_sleep_minute = temp_sleep.0 as u32;
            sleepy_id = id;
        }
    }
    println!("sleepy id strat 2: {}", sleepy_id);
    println!("sleepiest minute strat 2: {}", max_sleep_minute);
    println!("ID x minute: {}", sleepy_id * max_sleep_minute);
}
