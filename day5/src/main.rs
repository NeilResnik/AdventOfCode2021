use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use itertools::Itertools;

type Point = (i32, i32);
type Line = (Point, Point);

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("No input file specified!");
        return;
    }

    let map = parse_file(&args[1]);
    let intersection_cnt = map
        .values()
        .fold(0, |acc, cnt| if cnt > &1 { acc + 1 } else { acc });
    println!("{}", intersection_cnt);
}

fn parse_file(filename: &str) -> HashMap<Point, i32> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().fold(HashMap::new(), |mut map, l| {
        let graph_line: Line = l
            .unwrap()
            .split("->")
            .map(|s1| {
                let p: Point = s1
                    .split(',')
                    .map(|s2| s2.trim().parse::<i32>().unwrap())
                    .collect_tuple()
                    .unwrap();
                p
            })
            .collect_tuple()
            .unwrap();

        insert_line_points(&graph_line, &mut map);
        map
    })
}

fn insert_line_points(line: &Line, map: &mut HashMap<Point, i32>) {
    todo!()
}
