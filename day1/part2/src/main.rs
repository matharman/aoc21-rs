use std::io::{BufRead, Result as IOResult};

fn get_depth_input(path: &str) -> IOResult<Vec<u32>> {
    let f = std::fs::File::open(path)?;
    let rdr = std::io::BufReader::new(f).lines();

    let mut data = Vec::new();
    for line in rdr {
        data.push(line?.parse().unwrap());
    }

    Ok(data)
}

fn get_sums(data: &Vec<u32>) -> Vec<u32> {
    data.windows(3).fold(Vec::new(), |mut acc: Vec<u32>, win| {
        acc.push(win.iter().sum());
        acc
    })
}

fn get_increased(path: &str) -> u32 {
    let data = get_depth_input(path).unwrap();
    let sums = get_sums(&data);

    sums[..sums.len() - 1]
        .iter()
        .zip(sums[1..].iter())
        .fold(0, |acc, x| {
            let (x0, x1) = x;
            acc + (*x0 < *x1) as u32
        })
}

fn main() {
    let sample = "day1/etc/sample.txt";
    let data = "day1/etc/depth.txt";

    println!("Increased for {}: {}", sample, get_increased(sample));
    println!("Increased for {}: {}", data, get_increased(data));
}
