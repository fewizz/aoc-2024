use std::{collections::{HashMap, HashSet}, io::Read};

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    let mut lines = input.lines();

    loop {
        let mut lock = true;
        let mut r = Vec::from([0, 0, 0, 0, 0]);
        for y in 0..7 {
            let line = lines.next().unwrap();

            for x in 0..5 {
                let ch = line.as_bytes()[x];

                if y == 0 && x == 0 && ch == b'.' { lock = false; }

                assert!(ch == b'#' || ch == b'.');
                if (lock && ch == b'#') || (!lock && ch == b'.') {
                    r[x] += 1;
                }
            }
        }

        if lock {
            locks.push(r);
        }
        else {
            keys.push(r);
        }

        if lines.next().is_none() { break; }
    }

    let mut result = 0;

    for k in keys {
        //result += locks.iter().filter(|l| **l == k).count();
        result += locks.iter().filter(|l| l.iter().zip(k.iter()).all(|(l, k)| k >= l)).count();
    }

    println!("{result}");

}