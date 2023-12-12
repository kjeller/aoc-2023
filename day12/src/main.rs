use itertools::repeat_n;
use itertools::Itertools;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Spring {
    Damaged,
    Operational,
    Unknown,
}

impl Spring {
    fn from_char(c: &char) -> Spring {
        match c {
            '#' => Self::Damaged,
            '.' => Self::Operational,
            '?' => Self::Unknown,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct SpringRecord {
    spring_row: Vec<Spring>,
    engineer_record: Vec<usize>,
}

fn parse() -> Vec<SpringRecord> {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(Result::unwrap)
        .map(|line| {
            let mut spring_record = SpringRecord {
                spring_row: Vec::new(),
                engineer_record: Vec::new(),
            };
            if let Some((l, r)) = line.split_once(char::is_whitespace) {
                spring_record.spring_row = l
                    .chars()
                    .into_iter()
                    .map(|c| Spring::from_char(&c))
                    .collect();
                spring_record.engineer_record = r
                    .split(',')
                    .into_iter()
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<usize>>();
            }
            spring_record
        })
        .collect::<Vec<SpringRecord>>()
}

fn part_1() -> u64 {
    let mut spring_record = parse();

    spring_record.iter_mut().fold(0, |sum, record| {
        let mut count = 0;
        let mut replace_pos = record
            .spring_row
            .iter()
            .enumerate()
            .filter(|(_, s)| **s == Spring::Unknown)
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();
        let unknown_cnt = replace_pos.len();

        let variants: Vec<Spring> = vec![Spring::Damaged, Spring::Operational];
        for combination in repeat_n(variants.iter(), unknown_cnt).multi_cartesian_product() {
            for (i, spring) in replace_pos.iter_mut().zip(combination.iter()) {
                record.spring_row[*i] = **spring;
            }
            let group_cnt = record
                .spring_row
                .iter()
                .group_by(|&x| *x)
                .into_iter()
                .filter(|(p, _)| p == &Spring::Damaged)
                .map(|(_, group)| group.count())
                .collect::<Vec<usize>>();

            if record.engineer_record == group_cnt {
                count += 1;
            }
        }
        count + sum
    })
}

fn part_2() -> u64 {
    0
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

#[cfg(test)]
mod tests {
    use super::*;
}
