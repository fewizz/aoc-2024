use std::{collections::{HashMap, HashSet}, io::Read};

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();

    let mut gates = HashMap::new();
    let mut values = HashMap::new();

    while let Some(line) = lines.next() {
        if line == "" { break; }
        let (wire, value) = line.split_once(": ").unwrap();
        values.insert(wire, value.parse::<u32>().unwrap() > 0);
    }

    while let Some(line) = lines.next() {
        let words: Vec<&str> = line.split(" ").collect();
        gates.insert(words[4], (words[0], words[1], words[2]));
    }

    while !gates.is_empty() {
        gates.retain(|k, v| {
            let left = values.get(v.0);
            let right = values.get(v.2);
            if left.is_some() && right.is_some() {
                let output = match v.1 {
                    "AND" => { left.unwrap() & right.unwrap() },
                    "XOR" => { left.unwrap() ^ right.unwrap() },
                    "OR" => { left.unwrap() | right.unwrap() },
                    _ => {panic!()}
                };
                values.insert(k, output);
                false
            }
            else {
                true
            }
        });
    }

    let mut result: usize = 0;

    for (k, v) in values {
        if !v || !k.starts_with("z") { continue; }
        let bit = k[1..].parse::<u32>().unwrap();
        result |= 1 << bit;
    }

    println!("{result}")

}