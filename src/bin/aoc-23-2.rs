use std::{collections::{HashMap, HashSet}, io::Read};

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut connections = HashMap::new();

    for line in input.lines() {
        let (l, r) = line.split_once('-').unwrap();
        connections.entry(l).or_insert_with(HashSet::new).insert(r);
        connections.entry(r).or_insert_with(HashSet::new).insert(l);
    }

    let mut lans: Vec<HashSet<&str>> = Vec::new();

    for (h, other_hosts) in connections.iter() {
        for l in lans.iter_mut() {
            if l.iter().all(|h| other_hosts.contains(*h)) {
                l.insert(h);
            }
        }
        lans.push(HashSet::from([*h]));
    }

    lans.sort_by(|a, b| a.len().cmp(&b.len()));
    let result = lans.pop().unwrap();
    let mut result: Vec<&str> = result.iter().map(|x|*x).collect();
    result.sort();
    let result = result.join(",");
    println!("{result}")
}