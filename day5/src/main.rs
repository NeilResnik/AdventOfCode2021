pub mod line;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

use advent_of_code2021_day5::line::{Line, Point};

fn debug_print(x_max: i32, y_max: i32, map: &HashMap<Point, i32>) {
    for y in 0..=y_max {
        for x in 0..=x_max {
            let p = Point::new(x, y);
            if map.contains_key(&p) {
                print!("{}", map[&p]);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No input file specified!");
        return;
    }

    let lines = parse_file(&args[1]);
    let p1_map = populate_graph_part_1(&lines);
    let p1_intersection_cnt = p1_map
        .values()
        .fold(0, |acc, cnt| if cnt > &1 { acc + 1 } else { acc });
    println!("{}", p1_intersection_cnt);

    let p2_map = populate_graph_part_2(&lines);
    let p2_intersection_cnt = p2_map
        .values()
        .fold(0, |acc, cnt| if cnt > &1 { acc + 1 } else { acc });
    println!("{}", p2_intersection_cnt);
}

fn parse_file(filename: &str) -> Vec<Line> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| {
            l.unwrap()
                .split("->")
                .map(|s1| {
                    s1.split(',')
                        .map(|s2| s2.trim().parse::<i32>().unwrap())
                        .collect_tuple::<(i32, i32)>()
                        .unwrap()
                        .into()
                })
                .collect_tuple::<(Point, Point)>()
                .unwrap()
                .into()
        })
        .collect()
}

fn populate_graph_part_1(lines: &Vec<Line>) -> HashMap<Point, i32> {
    let mut map = HashMap::new();

    for l in lines {
        insert_line_points_part1(l, &mut map);
    }

    map
}

fn populate_graph_part_2(lines: &Vec<Line>) -> HashMap<Point, i32> {
    let mut map = HashMap::new();

    for l in lines {
        insert_line_points_part2(l, &mut map);
    }

    map
}

fn insert_line_points_part1(line: &Line, map: &mut HashMap<Point, i32>) {
    let x_diff = line.p2.x - line.p1.x;
    let y_diff = line.p2.y - line.p1.y;
    let pnt_cnt = x_diff.abs().max(y_diff.abs()) + 1;

    if (x_diff == 0 && y_diff != 0) || (x_diff != 0 && y_diff == 0) {
        let initial = if x_diff == 0 {
            Point::new(line.p1.x, line.p1.y.min(line.p2.y))
        } else {
            Point::new(line.p1.x.min(line.p2.x), line.p1.y)
        };

        for z in 0..pnt_cnt {
            let p: Point = if y_diff == 0 {
                Point::new(initial.x + z, initial.y)
            } else {
                Point::new(initial.x, initial.y + z)
            };
            // println!("{:?}", p);

            *map.entry(p).or_insert(0) += 1;
        }
    }
}

fn insert_line_points_part2(line: &Line, map: &mut HashMap<Point, i32>) {
    let x_diff = line.p2.x - line.p1.x;
    let y_diff = line.p2.y - line.p1.y;
    let pnt_cnt = x_diff.abs().max(y_diff.abs()) + 1;

    let x_delta = if line.p2.x > line.p1.x {
        1
    } else if line.p2.x < line.p1.x {
        -1
    } else {
        0
    };

    let y_delta = if line.p2.y > line.p1.y {
        1
    } else if line.p2.y < line.p1.y {
        -1
    } else {
        0
    };

    // println!("x_delta: {}, y_delta: {}", x_delta, y_delta);

    for z in 0..pnt_cnt {
        let p = Point::new(line.p1.x + (x_delta * z), line.p1.y + (y_delta * z));
        // println!("{:?}", p);
        *map.entry(p).or_insert(0) += 1;
    }
}
