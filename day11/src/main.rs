use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use pathfinding::directed::dijkstra::dijkstra;

const SIZE: usize = 140;

#[derive(Debug)]
struct Image {
    start: Vec<(isize, isize)>,
}

impl Image {
    fn new() -> Image {
        Image { start: Vec::new() }
    }

    fn get_valid_paths(&mut self, (x, y): (isize, isize)) -> [((isize, isize), isize); 4] {
        let weight = 1;
        let paths: [((isize, isize), isize); 4] = [
            ((x - 1, y), weight),
            ((x + 1, y), weight),
            ((x, y - 1), weight),
            ((x, y + 1), weight),
        ];
        paths
    }
}

fn parse() -> Image {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);
    let mut image = Image::new();
    reader
        .lines()
        .map(Result::unwrap)
        .enumerate()
        .for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, c)| match c {
                '#' => image.start.push((col as isize, row as isize)),
                _ => (),
            });
        });
    image
}

fn part_1() -> u64 {
    let mut image = parse();
    let mut start = image.start.clone();

    let expand_y = (0..SIZE)
        .into_iter()
        .filter(|f| !start.iter().map(|f| f.1).any(|s| *f as isize == s))
        .collect::<Vec<usize>>();

    expand_y.iter().rev().for_each(|f| {
        start
            .iter_mut()
            .filter(|s| s.1 > *f as isize)
            .for_each(|s| {
                s.1 += 1;
            });
    });
    let expand_x = (0..SIZE)
        .into_iter()
        .filter(|f| !start.iter().map(|f| f.0).any(|s| *f as isize == s))
        .collect::<Vec<usize>>();

    expand_x.iter().rev().for_each(|f| {
        start
            .iter_mut()
            .filter(|s| s.0 > *f as isize)
            .for_each(|s| {
                s.0 += 1;
            });
    });

    start
        .iter()
        .enumerate()
        .map(|(i, s1)| {
            start[i..start.len()]
                .iter()
                .map(|s2| {
                    if s1 != s2 {
                        let res = dijkstra(s1, |p| image.get_valid_paths(*p), |p| *p == *s2);
                        let res = match res {
                            Some(x) => x.1 as u64,
                            None => 0,
                        };
                        res
                    } else {
                        0
                    }
                })
                .sum::<u64>()
        })
        .sum()
}

fn part_2() -> i64 {
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
