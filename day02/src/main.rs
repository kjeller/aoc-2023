use parse_display::{Display, FromStr};
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

#[derive(Display, FromStr, PartialEq, Debug)]
#[display("Game {id}")]
#[from_str(default)]
struct Game {
    id: u64,
    is_valid: bool,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            id: 0,
            is_valid: false,
        }
    }
}

#[derive(Display, FromStr, PartialEq, PartialOrd, Debug)]
enum Color {
    #[display("{0} red")]
    Red(u64),
    #[display("{0} green")]
    Green(u64),
    #[display("{0} blue")]
    Blue(u64),
}

#[derive(PartialEq, PartialOrd, Debug)]
struct ColorBag {
    red: Color,
    green: Color,
    blue: Color,
}

impl ColorBag {
    fn copy_if_not_within(&mut self, colorbag: &ColorBag) {
        if !colorbag.is_within(&self) {
            if colorbag.red > self.red {
                self.copy_color(&colorbag.red)
            }
            if colorbag.green > self.green {
                self.copy_color(&colorbag.green)
            }
            if colorbag.blue > self.blue {
                self.copy_color(&colorbag.blue)
            }
        }
    }

    fn copy_color(&mut self, color: &Color) {
        match color {
            Color::Red(x) => self.red = Color::Red(*x),
            Color::Blue(x) => self.blue = Color::Blue(*x),
            Color::Green(x) => self.green = Color::Green(*x),
        }
    }

    fn is_within(&self, colorbag: &ColorBag) -> bool {
        return self.red <= colorbag.red
            && self.green <= colorbag.green
            && self.blue <= colorbag.blue;
    }

    fn power(&self) -> u64 {
        return match (&self.red, &self.green, &self.blue) {
            (Color::Red(red), Color::Green(green), Color::Blue(blue)) => red * green * blue,
            (_, _, _) => 0,
        };
    }
}
impl Default for ColorBag {
    fn default() -> Self {
        Self {
            red: Color::Red(0),
            green: Color::Green(0),
            blue: Color::Blue(0),
        }
    }
}

fn part_1() -> u64 {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);

    let rule = ColorBag {
        red: Color::Red(12),
        green: Color::Green(13),
        blue: Color::Blue(14),
    };

    let mut sum = 0;
    reader.lines().for_each(|line| {
        if let Ok(l) = line {
            if let Some((game_id, game_str)) = l.split_once(':') {
                let mut game = match Game::from_str(game_id) {
                    Ok(x) => x,
                    Err(_) => Game::default(),
                };

                let mut colorbag = ColorBag::default();
                for turn in game_str.split(";") {
                    turn.split(",").into_iter().for_each(|t| {
                        if let Ok(color) = Color::from_str(t.trim_start()) {
                            colorbag.copy_color(&color)
                        }
                    });
                    game.is_valid = colorbag.is_within(&rule);
                    if !game.is_valid {
                        break;
                    } else {
                    }
                }
                if game.is_valid {
                    sum += game.id;
                }
            }
        } else {
        }
    });
    return sum;
}

fn part_2() -> u64 {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            if let Ok(l) = line {
                if let Some((game_id, game_str)) = l.split_once(':') {
                    let mut _game = match Game::from_str(game_id) {
                        Ok(x) => x,
                        Err(_) => Game::default(),
                    };

                    let mut rule = ColorBag::default();
                    let mut colorbag = ColorBag::default();
                    for turn in game_str.split(";") {
                        turn.split(",").into_iter().for_each(|t| {
                            if let Ok(color) = Color::from_str(t.trim_start()) {
                                colorbag.copy_color(&color)
                            }
                        });
                        rule.copy_if_not_within(&colorbag);
                    }
                    rule.power()
                } else {
                    0
                }
            } else {
                0
            }
        })
        .sum::<u64>()
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
