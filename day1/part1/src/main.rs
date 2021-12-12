#![allow(dead_code)]

use std::io::{BufRead, Result as IOResult};
use std::path::PathBuf;

fn get_depth_input(path: PathBuf) -> IOResult<Vec<u32>> {
    let f = std::fs::File::open(path)?;
    let rdr = std::io::BufReader::new(f).lines();

    let mut data = Vec::new();
    for line in rdr {
        data.push(line?.parse().unwrap());
    }

    Ok(data)
}

fn get_increased_from_file(path: &str) -> u32 {
    let path = std::path::PathBuf::from(path);
    let data = get_depth_input(path).unwrap();

    let mut increased: u32 = 0;

    for i in 1..data.len() {
        if data[i - 1] < data[i] {
            increased += 1;
        }
    }

    increased
}

fn main() {
    let sample = "day1/part1/sample.txt";
    let data = "day1/part1/depth.txt";
    println!(
        "increased for {}: {}",
        sample,
        get_increased_from_file(sample)
    );

    println!("increased for {}: {}", data, get_increased_from_file(data));
}
