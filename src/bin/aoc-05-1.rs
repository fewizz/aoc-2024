use std::{collections::HashMap, io::Read};

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut result: i64 = 0;
    let mut iter = input.lines();
    let mut mapping: HashMap<i8, Vec<i8>> = HashMap::new();

    while let Some(line) = iter.next() {
        if line.is_empty() { break; }
        let (l, r) = line.split_once("|").unwrap();
        let entry = mapping.entry(l.parse::<i8>().unwrap());
        entry.or_insert_with(Vec::new).push(r.parse::<i8>().unwrap());
    }

    'outer: for line in iter {
        let nums: Vec<i8> = line.split(",").map(|num_str| num_str.parse().unwrap()).collect();

        for x in 1..nums.len() {
            for y in 0..x {
                let list = match mapping.get(&nums[x]) {
                    Some(list) => list, None => continue
                };
                if list.contains(&nums[y]) {
                    continue 'outer;
                }
            }
        }
        result += nums[nums.len() / 2] as i64;
    }

    println!("{result}");
}