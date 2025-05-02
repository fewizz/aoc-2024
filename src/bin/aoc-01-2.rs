use std::{collections::HashMap, io::Read};

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut left: Vec<i32> = Vec::new();
    let mut right: HashMap<i32, i32> = HashMap::new();
    for line in input.lines() {
        let (l, r) = line.split_once("   ").unwrap();
        let l = l.parse::<i32>().unwrap();
        let r = r.parse::<i32>().unwrap();

        left.push(l);
        right.entry(r).and_modify(|x| *x += 1).or_insert(1);
    }

    let mut result = 0;
    for l in left {
        result += l * right.get(&l).unwrap_or(&0);
    }
    println!("{result}");
}