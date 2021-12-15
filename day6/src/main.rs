pub mod lanternfish;

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use day6::lanternfish::LanternFish;

const DAY_COUNT: usize = 80;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No input file specified");
        return;
    }
    let mut fish = parse_file(&args[1]);
    simulate(&mut fish, DAY_COUNT);
    println!("{}", fish.len());
}

fn parse_file(filename: &str) -> Vec<LanternFish> {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();

    contents
        .split(',')
        .map(|s| LanternFish::new(s.trim().parse::<i32>().unwrap()))
        .collect()
}

fn simulate(fish: &mut Vec<LanternFish>, day_cnt: usize) {
    let mut new_fish = Vec::new();
    for _day in 0..day_cnt {
        for i in 0..fish.len() {
            if let Some(mut children) = fish[i].maybe_reproduce() {
                new_fish.append(&mut children);
            }
        }
        fish.append(&mut new_fish);
        new_fish.clear();
    }
}
