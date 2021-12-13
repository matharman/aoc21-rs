use file_utils::LineReader;

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
    aim: i32,
    horizontal: i32,
    depth: i32,
}

impl Position {
    fn apply_instruction(&mut self, inst: &Instruction) {
        match inst.dir {
            Direction::Up => self.aim -= inst.delta,
            Direction::Down => self.aim += inst.delta,
            Direction::Forward => {
                self.horizontal += inst.delta;
                self.depth += self.aim * inst.delta;
            }
        }
    }

    fn apply_course_from_file(&mut self, rdr: &mut LineReader) {
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

fn display_result(rdr: &mut LineReader) {
    let mut position = Position::default();
    position.apply_course_from_file(rdr);
    println!(
        "Final for {}: {:?}, {}",
        rdr.name(),
        position,
        position.depth * position.horizontal
    );
}

fn main() {
    let mut sample = file_utils::LineReader::open("day2/etc/sample.txt").unwrap();
    let mut directions = file_utils::LineReader::open("day2/etc/directions.txt").unwrap();

    display_result(&mut sample);
    display_result(&mut directions);
}
