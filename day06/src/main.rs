use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Race {
    time: u64,
    distance: u64,
}

fn parse_part_1() -> Vec<Race> {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(Result::unwrap).collect();
    lines
        .chunks(2)
        .flat_map(|s| {
            s.get(0)
                .unwrap()
                .as_str()
                .split_whitespace()
                .filter(|f| f.chars().nth(0).unwrap().is_numeric())
                .map(|f| f.parse::<u64>().unwrap())
                .zip(
                    s.get(1)
                        .unwrap()
                        .as_str()
                        .split_whitespace()
                        .filter(|f| f.chars().nth(0).unwrap().is_numeric())
                        .map(|f| f.parse::<u64>().unwrap()),
                )
                .map(|(time, distance)| Race { time, distance })
                .collect::<Vec<Race>>()
        })
        .collect::<Vec<Race>>()
}

fn parse_part_2() -> Race {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(Result::unwrap).collect();
    let num = lines
        .into_iter()
        .map(|l| {
            l.split_whitespace()
                .collect::<Vec<_>>()
                .join("")
                .split_once(':')
                .unwrap()
                .1
                .to_string()
        })
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    Race {
        time: *num.get(0).unwrap(),
        distance: *num.get(1).unwrap(),
    }
}

fn part_1() -> u64 {
    let races = parse_part_1();
    races
        .into_iter()
        .map(|f| {
            (0..=f.time)
                .into_iter()
                .map(|hold_speed| {
                    let distance = (f.time - hold_speed) * hold_speed;
                    (distance, hold_speed)
                })
                .filter(|(distance, _)| distance > &f.distance)
                .map(|(_, speed)| speed)
                .count() as u64
        })
        .product()
}

fn part_2() -> u64 {
    let race = parse_part_2();
    let mut counter = 0;
    for hold_speed in 0..=race.time {
        let distance = (race.time - hold_speed) * hold_speed;
        if distance > race.distance {
            counter += 1;
        }
    }
    counter
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
