use std::io::Read;

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

    fn check(towel: &str, patterns: &Vec<String>) -> bool {
        for p in patterns {
            if towel.starts_with(p) {
                let next = &towel[p.len()..];
                if next.len() == 0 {
                    return true;
                }
                if check(next, patterns) {
                    return true;
                }
            }
        }
        return false;
    }

    let mut result = 0;

    while let Some(towel) = iter.next() {
        if check(towel, &patterns) {
            result += 1;
        }
    }

    println!("{result}");
}