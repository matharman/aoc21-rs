use file_utils::LineReader;

fn get_bit_frequency_from_file<const N: usize>(path: &str) -> [i32; N] {
    let rdr = LineReader::open(path).unwrap();

    let mut bits: [i32; N] = [0; N];
    rdr.for_each(|x| {
        let bytes = u32::from_str_radix(x.unwrap().as_str(), 2).unwrap();
        for bit in 0..N {
            if bytes & (1 << bit) > 0 {
                bits[N - 1 - bit] += 1;
            } else {
                bits[N - 1 - bit] -= 1;
            }
        }
    });

    bits
}

fn get_gamma_from_file<const N: usize>(path: &str) -> u32 {
    let bits: [i32; N] = get_bit_frequency_from_file(path);
    bits.iter().enumerate().fold(0, |acc, tuple| {
        let (i, bit) = tuple;
        if *bit > 0 {
            acc | (1 << (N - 1 - i))
        } else {
            acc & !(1 << (N - 1 - i))
        }
    })
}

fn get_epsilon_from_file<const N: usize>(path: &str) -> u32 {
    let bits: [i32; N] = get_bit_frequency_from_file(path);
    bits.iter().enumerate().fold(0, |acc, tuple| {
        let (i, bit) = tuple;
        if *bit < 0 {
            acc | (1 << (N - 1 - i))
        } else {
            acc & !(1 << (N - 1 - i))
        }
    })
}

fn main() {
    let sample = "day3/etc/sample.txt";
    println!("Gamma for {}: {}", sample, get_gamma_from_file::<5>(sample));
    println!(
        "Epsilon for {}: {}",
        sample,
        get_epsilon_from_file::<5>(sample)
    );

    let data = "day3/etc/data.txt";
    println!("Gamma for {}: {}", data, get_gamma_from_file::<12>(data));
    println!(
        "Epsilon for {}: {}",
        data,
        get_epsilon_from_file::<12>(data)
    );
}
