use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use advent_of_code2021_day4::board::{Board, BoardError};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No input file specified!");
        return;
    }
    match parse_file(&args[1]) {
        Ok((boards, numbers)) => {
            println!("{}", get_winning_sum_part_1(boards.clone(), &numbers));
            println!("{}", get_winning_sum_part_2(boards, &numbers));
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn parse_file(filename: &str) -> Result<(Vec<Board>, Vec<i32>), String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let value_lines: Vec<String> = reader
        .lines()
        .filter_map(|s| {
            let s: String = s.unwrap().trim().into();
            if !s.is_empty() {
                Some(s)
            } else {
                None
            }
        })
        .collect();

    let numbers = parse_numbers(&value_lines[0]);
    let mut boards: Vec<Board> = Vec::new();

    let mut idx = 1;
    while idx + 5 <= value_lines.len() {
        boards.push(parse_board(&value_lines[idx..idx + 5])?);
        idx += 5;
    }
    Ok((boards, numbers))
}

fn parse_numbers(guesses: &String) -> Vec<i32> {
    guesses
        .split(',')
        .into_iter()
        .map(|vs| vs.parse().unwrap())
        .collect()
}

fn parse_board(board_strs: &[String]) -> Result<Board, BoardError> {
    let mut board_vec: Vec<Vec<i32>> = Vec::new();
    for l in board_strs {
        board_vec.push(
            l.split_whitespace()
                .into_iter()
                .map(|vs| vs.parse().unwrap())
                .collect(),
        );
    }

    Board::new(board_vec)
}

fn get_winning_sum_part_1(mut boards: Vec<Board>, numbers: &Vec<i32>) -> i32 {
    for n in numbers {
        for idx in 0..boards.len() {
            boards[idx].mark(*n);
            if boards[idx].is_winner() {
                return boards[idx].get_sum(*n);
            }
        }
    }
    return 0;
}

fn get_winning_sum_part_2(mut boards: Vec<Board>, numbers: &Vec<i32>) -> i32 {
    for n in numbers {
        for idx in 0..boards.len() {
            boards[idx].mark(*n);
        }
        if boards.len() == 1 && boards.first().unwrap().is_winner() {
            return boards.first().unwrap().get_sum(*n);
        }
        boards.retain(|b| !b.is_winner());
    }
    return 0;
}
