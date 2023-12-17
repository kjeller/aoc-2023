use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn hash(current_value: u64, input: &str) -> u64 {
    input
        .chars()
        .fold(current_value, |acc, c| ((acc + c as u64) * 17) % 256)
}

fn part_1() -> u64 {
    let file = File::open("input.txt").expect("not found");
    BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .fold(0, |acc, line| {
            line.split(',').map(|s| hash(0, s) as u64).sum::<u64>() + acc
        })
}

fn part_2() -> u64 {
    let file = File::open("input.txt").expect("not found");
    let mut map: HashMap<u64, Vec<(String, u64)>> = HashMap::new();
    BufReader::new(file)
        .lines()
        .map(Result::unwrap)
        .for_each(|line| {
            line.split(',').for_each(|s| {
                let (key, val) = s.split_once(|c: char| c.is_ascii_punctuation()).unwrap();
                let hash = hash(0, key);
                if let Some(entry) = map.get_mut(&hash) {
                    // split at '-'
                    if val.is_empty() {
                        if let Some(pos) = entry.iter().position(|x| *x.0 == *key) {
                            entry.remove(pos);
                        }
                    }
                    // split at '='
                    else {
                        let element = (key.to_string(), val.parse::<u64>().unwrap());
                        if let Some(pos) = entry.iter().position(|x| *x.0 == *key) {
                            entry[pos] = element;
                        } else {
                            entry.push(element)
                        }
                    }
                } else {
                    // split at '-'
                    if !val.is_empty() {
                        let vec = vec![(key.to_string(), val.parse::<u64>().unwrap())];
                        map.insert(hash, vec);
                    }
                }
            })
        });
    map.iter().fold(0, |acc: u64, (box_index, vec)| {
        vec.iter()
            .enumerate()
            .fold(0, |acc: u64, (slot_index, (_, focal_len))| {
                acc + (box_index + 1) * (slot_index as u64 + 1) * focal_len
            })
            + acc
    }) as u64
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
