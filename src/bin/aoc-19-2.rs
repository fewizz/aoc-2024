use std::{collections::HashMap, io::Read};

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut iter = input.lines();

    let mut patterns: Vec<String> = Vec::new();

    while let Some(line) = iter.next() {
        if line.len() == 0 { break; }
        for pattern in line.split(" ") {
            let mut pattern = pattern;
            if pattern.ends_with(",") {
                pattern = &pattern[..pattern.len()-1];
            }
            patterns.push(pattern.to_string());
        }
    }

    fn check(
        towel: &str, patterns: &Vec<String>, cache: &mut HashMap<String, usize>
    ) -> usize {
        let mut result: usize = 0;
        let cached = cache.get(towel);
        if cached.is_some() {
            return *cached.unwrap();
        }

        for p in patterns {
            if towel.starts_with(p) {
                let next = &towel[p.len()..];
                if next.len() == 0 {
                    result += 1;
                }
                else {
                    result += check(next, patterns, cache);
                }
            }
        }

        cache.insert(towel.to_string(), result);

        return result;
    }

    // Just throw some cache in! (tm)
    let mut cache: HashMap<String, usize> = HashMap::new();
    let mut result = 0;

    while let Some(towel) = iter.next() {
        let r = check(towel, &patterns, &mut cache);
        result += r;
    }

    println!("{result}");
}