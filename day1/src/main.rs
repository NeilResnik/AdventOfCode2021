use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }
    match parse_file(&args[1]) {
        Ok(measurements) => {
            println!("{}", calculate_increases_part_1(&measurements));
            println!("{}", calculate_increases_part_2(&measurements));
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn parse_file(filename: &str) -> std::io::Result<Vec<i32>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .map(|s| s.unwrap().trim().parse().unwrap())
        .collect())
}

fn calculate_increases_part_1(measurements: &Vec<i32>) -> i32 {
    measurements
        .iter()
        .tuple_windows::<(_, _)>()
        .fold(
            0,
            |acc, window| if window.1 > window.0 { acc + 1 } else { acc },
        )
}

fn calculate_increases_part_2(measurements: &Vec<i32>) -> i32 {
    measurements
        .iter()
        .tuple_windows::<(_, _, _)>()
        .map(|window| window.0 + window.1 + window.2)
        .tuple_windows::<(_, _)>()
        .fold(
            0,
            |acc, window| if window.1 > window.0 { acc + 1 } else { acc },
        )
}
