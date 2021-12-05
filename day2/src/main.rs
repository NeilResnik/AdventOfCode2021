use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

enum Direction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No input file specified!");
        return;
    }
    match parse_file(&args[1]) {
        Ok(instructions) => {
            println!("{}", calculate_postion_part_1(&instructions));
            println!("{}", calculate_postion_part_2(&instructions));
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn parse_file(filename: &str) -> std::io::Result<Vec<Direction>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    Ok(reader
        .lines()
        .map(|line| parse_instruction(line.unwrap().trim_end()).unwrap())
        .collect())
}

fn parse_instruction(instr: &str) -> Result<Direction, String> {
    let split: Vec<&str> = instr.split_whitespace().collect();
    if split.len() != 2 {
        Err(format!("Invalid Instruction: {}", instr))
    } else {
        let raw_dir = split[0].to_lowercase();
        let raw_val = split[1];
        let val = raw_val.parse::<i32>().unwrap();
        match raw_dir.as_str() {
            "forward" => Ok(Direction::Forward(val)),
            "down" => Ok(Direction::Down(val)),
            "up" => Ok(Direction::Up(val)),
            _ => Err(format!("Invalid Instruction: {}", instr)),
        }
    }
}

fn calculate_postion_part_1(instructions: &Vec<Direction>) -> i32 {
    let (horizontal, depth) = instructions.iter().fold((0, 0), |(h, d), dir| match dir {
        Direction::Forward(value) => (h + value, d),
        Direction::Down(value) => (h, d + value),
        Direction::Up(value) => (h, d - value),
    });

    horizontal * depth
}

fn calculate_postion_part_2(instructions: &Vec<Direction>) -> i32 {
    let (horizontal, depth, _aim) =
        instructions
            .iter()
            .fold((0, 0, 0), |(h, d, a), dir| match dir {
                Direction::Forward(value) => (h + value, d + (value * a), a),
                Direction::Down(value) => (h, d, a + value),
                Direction::Up(value) => (h, d, a - value),
            });

    horizontal * depth
}
