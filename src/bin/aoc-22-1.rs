use std::io::Read;

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut result: u64 = 0;

    for line in input.lines() {
        let mut secret = line.parse::<u64>().unwrap();

        for _ in 0..2000 {
            secret ^= secret * 64;
            secret %= 16777216;

            secret ^= secret / 32;
            secret %= 16777216;

            secret ^= secret * 2048;
            secret %= 16777216;
        }

        result += secret;
    }

    println!("{result}");
}