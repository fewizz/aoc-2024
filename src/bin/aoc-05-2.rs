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

    for line in iter {
        let mut nums: Vec<i8> = line.split(",").map(|num_str| num_str.parse().unwrap()).collect();

        let mut mutated = false;

        for y in 0..nums.len() {
            for x in y..nums.len() {
                let list = match mapping.get_mut(&nums[x]) {
                    Some(list) => list, None => continue
                };
                if list.contains(&nums[y]) {
                    nums.swap(y, x);
                    mutated = true;
                }
            }
        }

        if mutated {
            result += nums[nums.len() / 2] as i64;
        }
    }

    println!("{result}");
}