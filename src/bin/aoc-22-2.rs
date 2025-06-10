use std::{collections::{HashMap, HashSet}, io::Read};

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut all_bananas_by_diff: HashMap<(i64, i64, i64, i64), u64> = HashMap::new();

    for line in input.lines() {
        let mut secret = line.parse::<u64>().unwrap();

        let mut diffs_met: HashSet<(i64, i64, i64, i64)> = HashSet::new();

        let mut prev_price;
        let mut diffs = (0, 0, 0, 0);
        for i in 0..2000 {
            prev_price = secret % 10;

            secret ^= secret * 64;
            secret %= 16777216;

            secret ^= secret / 32;
            secret %= 16777216;

            secret ^= secret * 2048;
            secret %= 16777216;

            let price = secret % 10;
            let price_diff: i64 = prev_price as i64 - price as i64;

            diffs = (diffs.1, diffs.2, diffs.3, price_diff);
            if i >= 4 {
                let d = diffs_met.get(&diffs);
                if d.is_none() {
                    diffs_met.insert(diffs);
                    *all_bananas_by_diff.entry(diffs).or_insert(0) += price;
                }
            }
        }
    }

    let result = all_bananas_by_diff.values().max().unwrap();

    println!("{result}");
}