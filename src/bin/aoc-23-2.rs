use std::{collections::{HashMap, HashSet}, io::Read};

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut connections = HashMap::new();

    for line in input.lines() {
        let (l, r) = line.split_once('-').unwrap();
        connections.entry(l.to_string()).or_insert_with(HashSet::new).insert(r.to_string());
        connections.entry(r.to_string()).or_insert_with(HashSet::new).insert(l.to_string());
    }

    let mut lans: Vec<HashSet<&String>> = Vec::new();

    for (h, other_hosts) in connections.iter() {
        for l in lans.iter_mut() {
            if l.iter().all(|h| other_hosts.contains(*h)) {
                l.insert(h);
            }
        }
        lans.push(HashSet::from([h]));
    }

    lans.sort_by(|a, b| a.len().cmp(&b.len()));
    let result = lans.pop().unwrap();
    let mut result: Vec<String> = result.iter().map(|x|(*x).clone()).collect();
    result.sort();
    let result = result.join(",");
    println!("{result}")
}