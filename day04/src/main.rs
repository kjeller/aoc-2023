use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn part_1() -> u32 {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (_, cards) = line.split_once(':').unwrap();
            let (l, r) = cards.split_once('|').unwrap();
            let win_cards: Vec<u32> = l
                .split_whitespace()
                .into_iter()
                .map(|c| c.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let cards: Vec<u32> = r
                .split_whitespace()
                .into_iter()
                .map(|c| c.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let mut score = 0;
            cards
                .into_iter()
                .filter(|c| win_cards.contains(c))
                .enumerate()
                .for_each(|(i, _)| {
                    if i > 1 {
                        score *= 2;
                    } else {
                        score += 1;
                    }
                });
            score
        })
        .sum()
}

fn part_2() -> u32 {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);
    let mut instance_counter: Vec<u32> = vec![];

    // Card #, matches
    let cards_copies: Vec<(u32, u32)> = reader
        .lines()
        .enumerate()
        .map(|(i, line)| {
            instance_counter.push(1);
            let line = line.unwrap();
            let (_, cards) = line.split_once(':').unwrap();
            let (l, r) = cards.split_once('|').unwrap();
            let win_cards: Vec<u32> = l
                .split_whitespace()
                .into_iter()
                .map(|c| c.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let cards: Vec<u32> = r
                .split_whitespace()
                .into_iter()
                .map(|c| c.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            (
                i as u32,
                cards.into_iter().filter(|c| win_cards.contains(c)).count() as u32,
            )
        })
        .collect();

    for (i, c) in cards_copies {
        let curr_card_amount = *instance_counter.get_mut(i as usize).unwrap();
        for j in i + 1..=i + c {
            if let Some(x) = instance_counter.get_mut(j as usize) {
                *x = *x + curr_card_amount;
            }
        }
    }
    instance_counter.into_iter().sum()
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
