use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

use parse_display::FromStr;

#[derive(Debug)]
enum Instruction {
    Right,
    Left,
}

impl Instruction {
    fn from_char(c: char) -> Instruction {
        match c {
            'R' => Instruction::Right,
            'L' => Instruction::Left,
            _ => Instruction::Left,
        }
    }
}

#[derive(Debug, FromStr)]
#[display("{root} = ({left}, {right})")]
struct Map {
    root: String,
    left: String,
    right: String,
}

fn parse() -> Vec<String> {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);
    reader.lines().map(Result::unwrap).collect::<Vec<String>>()
}

fn part_1() -> u64 {
    let lines = parse();
    let mut line_iter = lines.iter();
    let instructions: Vec<Instruction> = line_iter
        .next()
        .unwrap()
        .chars()
        .map(|c| Instruction::from_char(c))
        .collect();
    println!("{:?}", instructions);
    line_iter.next(); // empty line

    let map = line_iter
        .map(|l| Map::from_str(l).unwrap())
        .collect::<Vec<Map>>();
    let mut path = HashMap::new();
    map.iter().for_each(|m| {
        path.insert(m.root.as_str(), (m.left.as_str(), m.right.as_str()));
    });
    let root = "AAA";
    let mut next: &str = root;
    let mut counter: u64 = 0;

    for i in instructions.iter().cycle() {
        let (left, right) = path[next];

        next = match i {
            Instruction::Left => left,
            Instruction::Right => right,
        };

        counter += 1;
        if next == "ZZZ" {
            break;
        }
    }
    counter
}

fn part_2() -> u64 {
    let lines = parse();
    let mut line_iter = lines.iter();
    let instructions: Vec<Instruction> = line_iter
        .next()
        .unwrap()
        .chars()
        .map(|c| Instruction::from_char(c))
        .collect();
    line_iter.next(); // empty line

    let map = line_iter
        .map(|l| Map::from_str(l).unwrap())
        .collect::<Vec<Map>>();
    let mut path = HashMap::new();
    map.iter().for_each(|m| {
        path.insert(m.root.as_str(), (m.left.as_str(), m.right.as_str()));
    });

    let mut steps_per_path: Vec<u64> = vec![];

    path.iter()
        .filter(|(key, _)| key.ends_with('A'))
        .for_each(|(key, _)| {
            let mut counter: u64 = 0;
            let mut next: &str = key;
            for i in instructions.iter().cycle() {
                let (left, right) = path[next];

                next = match i {
                    Instruction::Left => left,
                    Instruction::Right => right,
                };

                counter += 1;
                if next.ends_with('Z') {
                    break;
                }
            }
            steps_per_path.push(counter);
        });
    steps_per_path
        .iter()
        .fold(1, |a, b| num::integer::lcm(a, *b))
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
