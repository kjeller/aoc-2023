#![feature(iter_map_windows)]
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use num::Zero;

fn parse() -> Vec<Vec<i64>> {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(Result::unwrap)
        .map(|l| {
            l.split_whitespace()
                .map(|l| l.parse::<i64>().ok().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>()
}

fn history(line: Vec<i64>) -> Vec<Vec<i64>> {
    let mut history: Vec<Vec<i64>> = vec![];
    history.push(line);
    let mut iteration = 0;
    loop {
        if history[iteration].iter().all(|f| f.is_zero()) {
            break;
        }
        history.push(
            history[iteration]
                .iter()
                .map_windows(|[x, y]| **y - **x)
                .collect::<Vec<i64>>(),
        );
        iteration += 1;
    }
    history
}

fn part_1() -> i64 {
    let lines = parse();
    lines.iter().fold(0, |total, line| {
        history(line.to_vec())
            .iter()
            .fold(0, |acc, f| f.last().unwrap() + acc)
            + total
    })
}

fn part_2() -> i64 {
    let lines = parse();
    lines.iter().fold(0, |total, line| {
        history(line.to_vec())
            .iter()
            .rev()
            .fold(0, |acc, f| f.first().unwrap() - acc)
            + total
    })
}

fn main() {
    let part = match env::var("part") {
        Ok(val) => val,
        Err(_e) => "part1".to_string(),
    };

    if part == "part1" {
        println!("{}", part_1());
    } else if part == "part2" {
        println!("{}", part_2());
    }
}
