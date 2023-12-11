use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
struct Image {
    galaxies: Vec<(u128, u128)>,
}

impl Image {
    fn new() -> Image {
        Image {
            galaxies: Vec::new(),
        }
    }
}

impl Image {
    fn sum_distances(&self) -> u128 {
        self.galaxies
            .iter()
            .enumerate()
            .map(|(i, s1)| {
                self.galaxies[i..self.galaxies.len()]
                    .iter()
                    .map(|s2| {
                        if s1 != s2 {
                            let res = s1.0.abs_diff(s2.0) + s1.1.abs_diff(s2.1);
                            res
                        } else {
                            0
                        }
                    })
                    .sum::<u128>()
            })
            .sum()
    }
}

fn parse(expand_size: u128) -> Image {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);
    let mut image = Image::new();
    let width = 140;
    reader
        .lines()
        .map(Result::unwrap)
        .enumerate()
        .for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, c)| match c {
                '#' => image.galaxies.push((col as u128, row as u128)),
                _ => (),
            });
        });
    let mut start = image.galaxies.clone();
    let expand_y = (0..width)
        .into_iter()
        .filter(|f| !start.iter().map(|f| f.1).any(|s| *f == s))
        .collect::<Vec<u128>>();

    expand_y.iter().rev().for_each(|f| {
        start.iter_mut().filter(|s| s.1 > *f).for_each(|s| {
            s.1 += expand_size;
        });
    });
    let expand_x = (0..width)
        .into_iter()
        .filter(|f| !start.iter().map(|f| f.0).any(|s| *f == s))
        .collect::<Vec<u128>>();
    expand_x.iter().rev().for_each(|f| {
        start.iter_mut().filter(|s| s.0 > *f as u128).for_each(|s| {
            s.0 += expand_size;
        });
    });
    image.galaxies = start;
    image
}

fn part_1() -> u128 {
    parse(1).sum_distances()
}

fn part_2() -> u128 {
    parse(999_999).sum_distances()
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
