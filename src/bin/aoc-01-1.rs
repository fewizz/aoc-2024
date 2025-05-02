use std::{io::Read, iter::zip};

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in input.lines() {
        let (l, r) = line.split_once("   ").unwrap();
        left.push(l.parse::<i32>().unwrap());
        right.push(r.parse::<i32>().unwrap());
    }
    left.sort();
    right.sort();

    let mut result = 0;
    for (l, r) in zip(left, right) {
        result += (l - r).abs();
    }
    println!("{result}");
}