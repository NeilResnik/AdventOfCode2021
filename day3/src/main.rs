use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No input file specified!");
        return;
    }
    match parse_file(&args[1]) {
        Ok(numbers) => {
            println!("{}", calculate_diagnostic_part_1(&numbers));
            println!("{}", calculate_diagnostic_part_2(&numbers));
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}

fn parse_file(filename: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    Ok(reader.lines().map(|s| s.unwrap()).collect())
}

fn get_one_counts(numbers: &Vec<String>) -> Vec<usize> {
    if numbers.is_empty() {
        return Vec::new();
    }

    let bit_cnt = numbers.first().unwrap().len();
    let mut one_count = vec![0; bit_cnt];
    for str_val in numbers {
        for pos in 0..bit_cnt {
            if str_val.chars().nth(pos).unwrap() == '1' {
                one_count[pos] += 1;
            }
        }
    }
    one_count
}

fn calculate_diagnostic_part_1(numbers: &Vec<String>) -> usize {
    if numbers.is_empty() {
        return 0;
    }
    let one_counts = get_one_counts(numbers);
    let total_nums = numbers.len();
    let bit_cnt = numbers.first().unwrap().len();

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    for pos in 0..one_counts.len() {
        if one_counts[pos] > total_nums - one_counts[pos] {
            gamma_rate |= 1 << (bit_cnt - pos - 1);
        } else {
            epsilon_rate |= 1 << (bit_cnt - pos - 1);
        }
    }

    gamma_rate * epsilon_rate
}

fn calculate_diagnostic_part_2(numbers: &Vec<String>) -> usize {
    if numbers.is_empty() {
        return 0;
    }

    let bit_cnt = numbers.first().unwrap().len();

    let mut oxygen_rating_numbers: Vec<String> = numbers.to_vec();
    let mut co2_rating_numbers: Vec<String> = numbers.to_vec();
    for pos in 0..bit_cnt {
        if oxygen_rating_numbers.len() > 1 {
            let one_cnt = get_one_counts(&oxygen_rating_numbers);
            let total = oxygen_rating_numbers.len();
            oxygen_rating_numbers = oxygen_rating_numbers
                .iter()
                .filter_map(|num| {
                    if num.chars().nth(pos).unwrap() == get_most_common(one_cnt[pos], total) {
                        Some(num.to_owned())
                    } else {
                        None
                    }
                })
                .collect();
        }
    }

    for pos in 0..bit_cnt {
        if co2_rating_numbers.len() > 1 {
            let one_cnt = get_one_counts(&co2_rating_numbers);
            let total = co2_rating_numbers.len();
            co2_rating_numbers = co2_rating_numbers
                .iter()
                .filter_map(|num| {
                    if num.chars().nth(pos).unwrap() == get_least_common(one_cnt[pos], total) {
                        Some(num.to_owned())
                    } else {
                        None
                    }
                })
                .collect();
        }
    }
    let oxygen_rating = usize::from_str_radix(oxygen_rating_numbers.first().unwrap(), 2).unwrap();
    let co2_rating = usize::from_str_radix(co2_rating_numbers.first().unwrap(), 2).unwrap();

    oxygen_rating * co2_rating
}

fn get_most_common(one_cnt: usize, total: usize) -> char {
    if one_cnt >= total - one_cnt {
        '1'
    } else {
        '0'
    }
}

fn get_least_common(one_cnt: usize, total: usize) -> char {
    if one_cnt < total - one_cnt {
        '1'
    } else {
        '0'
    }
}
