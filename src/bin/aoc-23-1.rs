use std::{collections::{HashMap, HashSet}, io::Read};

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut connections = HashMap::new();

    for line in input.lines() {
        let (l, r) = line.split_once('-').unwrap();
        connections.entry(l).or_insert_with(Vec::new).push(r);
        connections.entry(r).or_insert_with(Vec::new).push(l);
    }

    let mut result = 0;
    let mut checked: HashSet<Vec<&str>> = HashSet::new();

    for (host_1, other_hosts) in connections.iter() {
        for host_2 in other_hosts.iter() {
            if host_2 == host_1 {continue;}
            for host_3 in other_hosts.iter() {
                if host_3 == host_1 || host_3 == host_1 {continue;}

                if !connections[host_2].contains(host_1) || !connections[host_2].contains(host_3) {continue;}
                if !connections[host_3].contains(host_1) || !connections[host_3].contains(host_2) {continue;}

                let mut hosts = vec![*host_1, *host_2, *host_3];

                hosts.sort();
                if
                    !checked.contains(&hosts) &&
                    hosts.iter().any(|h| h.starts_with('t'))
                {
                    result += 1;
                    checked.insert(hosts);
                }
            }
        }
    }

    println!("{result}");
}