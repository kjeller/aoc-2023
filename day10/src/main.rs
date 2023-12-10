use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use pathfinding::directed::dijkstra::dijkstra_all;

const SIZE: usize = 140;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Pipe {
    Vertical,
    Horizontal,
    NeBend,
    NwBend,
    SwBend,
    SeBend,
    Ground,
    Start,
}

impl Pipe {
    fn from_char(c: &char) -> Pipe {
        match c {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NeBend,
            'J' => Self::NwBend,
            '7' => Self::SwBend,
            'F' => Self::SeBend,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => unimplemented!(),
        }
    }
}
impl Pipe {}

#[derive(Debug)]
struct PipeMaze {
    maze: [Pipe; SIZE * SIZE],
    start: (isize, isize),
}

impl PipeMaze {
    fn new() -> PipeMaze {
        PipeMaze {
            maze: [Pipe::Start; SIZE * SIZE],
            start: (0, 0),
        }
    }

    fn set(&mut self, x: isize, y: isize, pipe: Pipe) {
        self.maze[x as usize + SIZE * y as usize] = pipe;
    }

    fn get(&mut self, (x, y): (isize, isize)) -> Pipe {
        self.maze[x as usize + SIZE * y as usize]
    }

    fn get_valid_paths(
        &mut self,
        current: &Pipe,
        (x, y): (isize, isize),
    ) -> Vec<((isize, isize), isize)> {
        let mut paths: Vec<((isize, isize), isize)> = Vec::new();
        let weight = 1;
        if x > 0 {
            paths.push(((x - 1, y), weight));
        }
        paths.push(((x + 1, y), weight));
        if y > 0 {
            paths.push(((x, y - 1), weight));
        }
        paths.push(((x, y + 1), weight));
        paths = paths
            .into_iter()
            .filter(|(f, _)| f.0 >= 0 && f.0 < SIZE as isize && f.1 >= 0 && f.1 < SIZE as isize)
            .filter(|(f, _)| {
                let other = self.get(*f);
                let res = Self::is_path_valid(current, &other, (x, y), *f);
                res
            })
            .collect();
        paths
    }

    fn is_path_valid(p1: &Pipe, p2: &Pipe, p1_pos: (isize, isize), p2_pos: (isize, isize)) -> bool {
        match (&p1, &p2) {
            (Pipe::Horizontal, Pipe::NwBend) | (Pipe::Horizontal, Pipe::SwBend) => {
                p1_pos.0 < p2_pos.0 && p1_pos.1 == p2_pos.1
            }
            (Pipe::Horizontal, Pipe::NeBend) | (Pipe::Horizontal, Pipe::SeBend) => {
                p1_pos.0 > p2_pos.0 && p1_pos.1 == p2_pos.1
            }
            (Pipe::Horizontal, Pipe::Horizontal) => p1_pos.1 == p2_pos.1,
            (Pipe::Vertical, Pipe::NeBend)
            | (Pipe::Vertical, Pipe::NwBend)
            | (Pipe::SwBend, Pipe::NwBend) => p1_pos.0 == p2_pos.0 && p1_pos.1 < p2_pos.1,
            (Pipe::Vertical, Pipe::SeBend)
            | (Pipe::Vertical, Pipe::SwBend)
            | (Pipe::NeBend, Pipe::SeBend) => p1_pos.0 == p2_pos.0 && p1_pos.1 > p2_pos.1,
            (Pipe::Vertical, Pipe::Vertical) => p1_pos.0 == p2_pos.0,
            (Pipe::NeBend, Pipe::NwBend) | (Pipe::SeBend, Pipe::SwBend) => {
                p1_pos.1 == p2_pos.1 && p1_pos.0 < p2_pos.0
            }
            (Pipe::NwBend, Pipe::SeBend) => {
                (p1_pos.1 == p2_pos.1 && p1_pos.0 > p2_pos.0)
                    || (p1_pos.0 == p2_pos.0 && p1_pos.1 > p2_pos.1)
            }
            (Pipe::SwBend, Pipe::NeBend) => {
                (p1_pos.0 == p2_pos.0 && p1_pos.1 < p2_pos.1)
                    || (p1_pos.1 == p2_pos.1 && p1_pos.0 > p2_pos.0)
            }
            (Pipe::NeBend, Pipe::NeBend)
            | (Pipe::NwBend, Pipe::NwBend)
            | (Pipe::SeBend, Pipe::SeBend)
            | (Pipe::SwBend, Pipe::SwBend)
            | (Pipe::Vertical, Pipe::Horizontal)
            | (_, Pipe::Ground)
            | (Pipe::Ground, _)
            | (Pipe::Start, Pipe::Start) => false,
            (Pipe::Start, _) => {
                Self::is_path_valid(&Pipe::Vertical, p2, p1_pos, p2_pos)
                    || Self::is_path_valid(&Pipe::Horizontal, p2, p1_pos, p2_pos)
            }
            (_, _) => Self::is_path_valid(p2, p1, p2_pos, p1_pos),
        }
    }
}

fn parse() -> PipeMaze {
    let file = File::open("input.txt").expect("not found");
    let reader = BufReader::new(file);
    let mut pipe_maze = PipeMaze::new();
    reader
        .lines()
        .map(Result::unwrap)
        .enumerate()
        .for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, c)| {
                let pipe = Pipe::from_char(&c);
                if pipe == Pipe::Start {
                    pipe_maze.start = (col as isize, row as isize);
                }
                pipe_maze.set(col as isize, row as isize, pipe)
            });
        });
    pipe_maze
}

fn part_1() -> i64 {
    let mut pipe_maze = parse();
    let start = pipe_maze.start.clone();

    dijkstra_all(&start, |p| {
        let current = pipe_maze.get(*p);
        pipe_maze.get_valid_paths(&current, *p)
    })
    .iter()
    .map(|(_, total)| total.1)
    .max()
    .unwrap() as i64
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

    #[test]
    fn vertical_vertical_pipe() {
        assert!(PipeMaze::is_path_valid(
            &Pipe::Vertical,
            &Pipe::Vertical,
            (1, 1),
            (1, 0)
        ));
        assert!(PipeMaze::is_path_valid(
            &Pipe::Vertical,
            &Pipe::Vertical,
            (1, 1),
            (1, 2)
        ));
        assert!(!PipeMaze::is_path_valid(
            &Pipe::Vertical,
            &Pipe::Vertical,
            (1, 1),
            (0, 1)
        ));
        assert!(!PipeMaze::is_path_valid(
            &Pipe::Vertical,
            &Pipe::Vertical,
            (1, 1),
            (2, 1)
        ));
    }

    #[test]
    fn nwbend_vertical_pipe() {
        assert!(!PipeMaze::is_path_valid(
            &Pipe::NeBend,
            &Pipe::Vertical,
            (1, 1),
            (1, 0)
        ));
        assert!(PipeMaze::is_path_valid(
            &Pipe::Vertical,
            &Pipe::Vertical,
            (1, 1),
            (1, 2)
        ));
        assert!(!PipeMaze::is_path_valid(
            &Pipe::Vertical,
            &Pipe::Vertical,
            (1, 1),
            (0, 1)
        ));
        assert!(!PipeMaze::is_path_valid(
            &Pipe::Vertical,
            &Pipe::Vertical,
            (1, 1),
            (2, 1)
        ));
    }

    #[test]
    fn nwbend_sebend() {
        assert!(!PipeMaze::is_path_valid(
            &Pipe::NeBend,
            &Pipe::SeBend,
            (1, 1),
            (1, 0)
        ));
        assert!(PipeMaze::is_path_valid(
            &Pipe::NeBend,
            &Pipe::SeBend,
            (1, 1),
            (1, 2)
        ));
        assert!(PipeMaze::is_path_valid(
            &Pipe::NeBend,
            &Pipe::SeBend,
            (1, 1),
            (0, 1)
        ));
        assert!(!PipeMaze::is_path_valid(
            &Pipe::NeBend,
            &Pipe::SeBend,
            (1, 1),
            (2, 1)
        ));
    }
}
