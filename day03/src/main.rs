#![feature(iter_map_windows)]
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use regex::Regex;

struct PartNumber {
    part_number: i64,
    line: Line,
}
struct Symbol {
    c: char,
    rect: Rectangle,
}

#[derive(PartialEq, PartialOrd, Clone)]
struct Coordinate {
    x: i64,
    y: i64,
}

struct Rectangle {
    top: Line,
    right: Line,
    bottom: Line,
    left: Line,
}

struct Line {
    a: Coordinate,
    b: Coordinate,
}

impl Rectangle {
    fn get_rect(coord: &Coordinate) -> Rectangle {
        let top_left = Coordinate {
            x: coord.x - 1,
            y: coord.y + 1,
        };
        let top_right = Coordinate {
            x: coord.x + 1,
            y: coord.y + 1,
        };
        let bot_left = Coordinate {
            x: coord.x - 1,
            y: coord.y - 1,
        };
        let bot_right = Coordinate {
            x: coord.x + 1,
            y: coord.y - 1,
        };
        Rectangle {
            top: Line {
                a: top_left.clone(),
                b: top_right.clone(),
            },
            right: Line {
                a: top_right.clone(),
                b: bot_right.clone(),
            },
            bottom: Line {
                a: bot_left.clone(),
                b: bot_right.clone(),
            },
            left: Line {
                a: top_left.clone(),
                b: bot_left.clone(),
            },
        }
    }
}

impl Line {
    fn intersect_dir(&self, other: &Self) -> bool {
        self.a.x <= other.a.x
            && self.b.x >= other.b.x
            && self.a.y <= other.a.y
            && self.b.y >= other.b.y
    }

    fn intersect(&self, other: &Self) -> bool {
        self.intersect_dir(other) || other.intersect_dir(self)
    }

    fn intersect_rect(&self, r: &Rectangle) -> bool {
        self.intersect(&r.top)
            || self.intersect(&r.right)
            || self.intersect(&r.bottom)
            || self.intersect(&r.left)
    }
}

fn parse(part_numbers: &mut Vec<PartNumber>) -> Vec<Symbol> {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);
    let re = Regex::new(r"\d+").unwrap();
    reader
        .lines()
        .enumerate()
        .map(|(y, l)| {
            let line = match l {
                Ok(l) => l,
                Err(_) => "".to_string(),
            };

            re.find_iter(line.as_str()).for_each(|m| {
                if let Ok(num) = m.as_str().parse::<i64>() {
                    part_numbers.push(PartNumber {
                        part_number: num,
                        line: Line {
                            a: Coordinate {
                                x: m.start() as i64,
                                y: y as i64,
                            },
                            b: Coordinate {
                                x: (m.end() - 1) as i64,
                                y: y as i64,
                            },
                        },
                    })
                }
            });
            line.chars()
                .enumerate()
                .filter(|(_, c)| !c.is_alphanumeric() && c != &'.')
                .map(|(x, c)| Symbol {
                    c,
                    rect: Rectangle::get_rect(&Coordinate {
                        x: x as i64,
                        y: y as i64,
                    }),
                })
                .collect::<Vec<Symbol>>()
        })
        .flat_map(|f| f)
        .collect::<Vec<Symbol>>()
}

fn part_1() -> u64 {
    let mut part_numbers: Vec<PartNumber> = vec![];
    let symbols = parse(part_numbers.as_mut());

    part_numbers
        .iter()
        .map(|p| {
            symbols
                .iter()
                .map(|s| {
                    if p.line.intersect_rect(&s.rect) {
                        p.part_number as u64
                    } else {
                        0 as u64
                    }
                })
                .sum::<u64>()
        })
        .sum::<u64>()
}

fn part_2() -> i64 {
    let mut part_numbers: Vec<PartNumber> = vec![];
    let symbols = parse(part_numbers.as_mut());
    symbols
        .iter()
        .filter(|s| match s.c {
            '*' => true,
            _ => false,
        })
        .map(|s| {
            let nums: Vec<i64> = part_numbers
                .iter()
                .filter(|p| p.line.intersect_rect(&s.rect))
                .map(|p| p.part_number)
                .collect();

            if nums.len() == 2 {
                nums.first().unwrap() * nums.last().unwrap()
            } else {
                0
            }
        })
        .sum::<i64>()
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

    #[test]
    fn line_line_intersect() {
        let a1 = Line {
            a: Coordinate { x: 0, y: 1 },
            b: Coordinate { x: 3, y: 1 },
        };
        let a2 = Line {
            a: Coordinate { x: 0, y: 0 },
            b: Coordinate { x: 4, y: 0 },
        };
        let b1 = Line {
            a: Coordinate { x: 3, y: 3 },
            b: Coordinate { x: 3, y: 1 },
        };

        assert!(a1.intersect(&a1));
        assert!(a1.intersect(&b1));
        assert!(b1.intersect(&a1));
        assert!(!a2.intersect(&a1));
        assert!(!a2.intersect(&b1));
    }

    #[test]
    fn line_rect_intersect() {
        let a = Line {
            a: Coordinate { x: 0, y: 0 },
            b: Coordinate { x: 5, y: 0 },
        };
        let b = Rectangle::get_rect(&Coordinate { x: 0, y: 0 });

        assert!(a.intersect_rect(&b));
    }
}
