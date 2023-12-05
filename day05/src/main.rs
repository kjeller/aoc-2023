use parse_display::Display;
use parse_display::FromStr;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::Range;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Display, FromStr, Eq, PartialEq, Hash, Clone, EnumIter)]
#[display(style = "lowercase")]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug, FromStr)]
#[display("{dest} {src} {range_len}")]
struct MapEntry {
    src: u64,
    dest: u64,
    range_len: u64,
}

struct MapRange {
    src: Range<u64>,
    dest: Range<u64>,
}

struct SeedMap {
    seeds: Vec<u64>,
    map: HashMap<Category, Vec<MapRange>>
} 

fn parse() -> SeedMap {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);
    let mut line_iterator = reader.lines();
    let mut map: HashMap<Category, Vec<MapRange>> = HashMap::new();
    map.insert(Category::Seed, Vec::new());

    let seeds = line_iterator
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .filter(|f| f.chars().nth(0).unwrap().is_numeric())
        .map(|f| f.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut prev_category = Category::Seed;
    line_iterator.for_each(|l| {
        let l = l.unwrap().to_string();
        if let Some((first, _)) = l.split_once("-to-") {
            if let Ok(category) = Category::from_str(first) {
                prev_category = category;
            }
        } else {
            if let Ok(e) = MapEntry::from_str(&l) {
                if map.get_key_value(&prev_category).is_none() {
                    map.insert(prev_category.clone(), Vec::new());
                }
                if let Some(entry) = map.get_mut(&prev_category) {
                    entry.push(MapRange {
                        src: e.src..(e.src + e.range_len),
                        dest: e.dest..(e.dest + e.range_len),
                    });
                }
            }
        }
    });

    SeedMap {
        seeds,
        map,
    }
}

fn part_1() -> u64 {
    let seed_map = parse();

    seed_map.seeds.iter().map(|s| {
        let s = s.clone();
        let min = Category::iter()
        .filter(|c| c != &Category::Location)
        .fold(s, |acc, c| {
            seed_map.map.get(&c).unwrap().into_iter().fold(acc, |acc2, f| {
                if f.src.contains(&acc) {
                    match acc2 >= f.dest.end && acc2 >= f.src.end {
                        true => acc2 - f.src.end - f.dest.end,
                        false => f.dest.end - (f.src.end - acc2),
                    }
                } else {
                    acc2
                }
            })
        });
        min
    }).min().unwrap()
}

fn part_2() -> u64 {
    let seed_map = parse();

    seed_map.seeds.windows(2).map(|s| {
        (s[0]..=(&s[0]+&s[1])).into_iter().map(|s| {
        let min = Category::iter()
        .filter(|c| c != &Category::Location)
        .fold(s, |acc, c| {
            seed_map.map.get(&c).unwrap().into_iter().fold(acc, |acc2, f| {
                if f.src.contains(&acc) {
                    match acc2 >= f.dest.end && acc2 >= f.src.end {
                        true => acc2 - f.src.end - f.dest.end,
                        false => f.dest.end - (f.src.end - acc2),
                    }
                } else {
                    acc2
                }
            })
        });
        min
        }).min().unwrap()
    }).min().unwrap()
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
