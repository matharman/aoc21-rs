#![allow(dead_code)]
use std::io::BufRead;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Forward,
}

impl From<&str> for Direction {
    fn from(x: &str) -> Self {
        match x {
            "up" => Direction::Up,
            "down" => Direction::Down,
            "forward" => Direction::Forward,
            _ => panic!("unknown direction"),
        }
    }
}

#[derive(Debug)]
struct Instruction {
    dir: Direction,
    delta: i32,
}

#[derive(Debug, Default)]
struct Position {
    horizontal: i32,
    depth: i32,
}

impl Position {
    fn apply_instruction(&mut self, inst: &Instruction) {
        match inst.dir {
            Direction::Up => self.depth -= inst.delta,
            Direction::Down => self.depth += inst.delta,
            Direction::Forward => self.horizontal += inst.delta,
        }
    }

    fn apply_course_from_file(&mut self, path: &str) {
        let f = std::fs::File::open(path).unwrap();
        let rdr = std::io::BufReader::new(f).lines();

        for line in rdr {
            let line = line.unwrap();
            let split: Vec<&str> = line.split_ascii_whitespace().collect();

            self.apply_instruction(&Instruction {
                dir: split[0].into(),
                delta: split[1].parse().unwrap(),
            });
        }
    }
}

fn display_result(path: &str) {
    let mut position = Position::default();
    position.apply_course_from_file(path);
    println!(
        "Final for {}: {:?}, {}",
        path,
        position,
        position.depth * position.horizontal
    );
}

fn main() {
    let sample = "day2/etc/sample.txt";
    let directions = "day2/etc/directions.txt";

    display_result(sample);
    display_result(directions);
}
