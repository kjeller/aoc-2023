use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use strum::IntoEnumIterator;
use strum_macros::Display;
use strum_macros::EnumIter;
use strum_macros::EnumString;

#[derive(Display, EnumIter, EnumString, PartialEq)]
#[allow(non_camel_case_types)]
enum Numeric {
    one = 1,
    two = 2,
    three = 3,
    four = 4,
    five = 5,
    six = 6,
    seven = 7,
    eight = 8,
    nine = 9,
}

fn part_1() -> u32 {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);

    let vec: Vec<Vec<u32>> = reader
        .lines()
        .map(|line| {
            if let Ok(l) = line {
                l.chars()
                    .filter(|c| c.is_numeric())
                    .map(|c| c.to_digit(10).unwrap() as u32)
                    .collect::<Vec<u32>>()
            } else {
                vec![0]
            }
        })
        .collect();
    return vec
        .iter()
        .map(|f| f.first().unwrap() * 10 + f.last().unwrap())
        .sum();
}

fn part_2() -> u64 {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);
    return reader
        .lines()
        .map(|line| {
            if let Ok(l) = line {
                let mut matches = vec![];
                let chars = l.chars();
                chars.enumerate().for_each(|(i, c)| {
                    if c.is_numeric() {
                        matches.push(c.to_digit(10).unwrap() as u64);
                    } else {
                        for n in Numeric::iter() {
                            let numstr = n.to_string();
                            let substring = l.get(i..i + numstr.len());
                            if let Some(s) = substring {
                                if s == n.to_string() {
                                    matches.push(n as u64);
                                    break;
                                }
                            }
                        }
                    }
                });
                if matches.len() > 0 {
                    matches.first().unwrap() * 10 as u64 + matches.last().unwrap() * 1 as u64
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum::<u64>();
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
