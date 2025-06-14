use core::fmt;
use std::{collections::{HashMap, HashSet}, io::Read};

/*

z0 = (x0 ^ y0)

z1 = (x1 ^ y1) ^ (
    x0 & y0 ─────────────────────┐
)                                │
                                 │
z2 = (x2 ^ y2) ^ (               │
    (x1 & y1) | ( ───────────────┼───┐
        (x1 ^ y1) & (            │   │
            x0 & y0 <────────────┘   │
        )                            │
    )                                │
)                                    │
                                     │
z3 = (x3 ^ y3) ^ (                   │
    (x2 & y2) | ( ───────────────────┼─────┐
        (x2 ^ y2) & (                │     │
            (x1 & y1) | ( <──────────┘     │
                (x1 ^ y1) & (              │
                    x0 & y0                │
                )                          │
            )                              │
        )                                  │
    )                                      │
)                                          │
                                           │
z4 = (x4 ^ y4) ^ (                         │
    (y3 & x3) | ( ─────────────────────────┼───────┐
        (x3 ^ y3) & (                      │       │
            (x2 & y2) | ( <────────────────┘       │
                (x2 ^ y2) & (                      │
                    (x1 & y1) | (                  │
                        (x1 ^ y1) & (              │
                            x0 & y0                │
                        )                          │
                    )                              │
                )                                  │
            )                                      │
        )                                          │
    )                                              │
)                                                  .
                                                   .
..etc                                              .

*/

// Dont ask me what is this...

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        if line == "" { break; }
    }

    let mut in2out = HashMap::new();
    let mut out2in = HashMap::new();

    while let Some(line) = lines.next() {
        let mut words: Vec<&str> = line.split(" ").collect();
        words.remove(3); // ->
        let op = words.remove(1);
        let output = words.remove(2);
        words.sort();

        in2out.insert((words[0], op, words[1]), output);
        out2in.insert(output, (words[0], op, words[1]));
    }

    fn print_gate(gates: &HashMap<&str, (&str, &str, &str)>, output: &str) {
        let (left, op, right) = gates[output];
        if left.starts_with("x") || left.starts_with("y") { print!("{left}") }
        else { print!("("); print_gate(gates, left); print!(")"); }
        print!(" {op} ");
        if right.starts_with("x") || right.starts_with("y") { print!("{right}") }
        else { print!("("); print_gate(gates, right); print!(")"); }
    }

    for g in out2in.keys() {
        if !g.starts_with("z") {continue;}
        print!("{g} = ");
        print_gate(&out2in, g);
        print!("\n\n\n\n");
    }

    let mut swaps = Vec::new();
    let mut prev_carry = String::new();

    for bit in 0..9000 {

        fn swap<'a>(
            in2out: &mut HashMap<
                (&'a str, &'a str, &'a str),
                &'a str
            >,
            out2in: &mut HashMap<
                &'a str,
                (&'a str, &'a str, &'a str)
            >,
            swaps: &mut Vec<String>,
            a: &str,
            b: &str
        ) {
            println!("swapping {a} and {b}");

            let a_key = *out2in.get(a).unwrap();
            let b_key = *out2in.get(b).unwrap();
            let a_value = *in2out.get(&a_key).unwrap();
            let b_value = *in2out.get(&b_key).unwrap();

            in2out.insert(a_key, b_value);
            in2out.insert(b_key, a_value);
            out2in.insert(b_value, a_key);
            out2in.insert(a_value, b_key);

            swaps.push(a_value.to_string());
            swaps.push(b_value.to_string());
        }

        fn get_output<'a>(
            gates: &'a HashMap<(&str, &str, &str), &str>,
            inputs: (&str, &str, &str)
        ) -> Option<&'a str> {
            let mut r = gates.get(&inputs);
            if r.is_none() {
                r = gates.get(&(inputs.2, inputs.1, inputs.0));
            }
            if r.is_none() {
                println!("couldn't find {inputs:?}");
            }
            return r.map(|v| *v);
        }

        let x = format!("x{bit:0>2}");
        let y = format!("y{bit:0>2}");
        let z = format!("z{bit:0>2}");
        let z = z.as_str();

        let main_key = (x.as_str(), "XOR", y.as_str());
        println!("{main_key:?}");
        let main = get_output(&in2out, main_key);
        if main.is_none() {
            break; // praying that last bit is right
        }
        let mut main = main.unwrap().to_string();

        if bit == 0 {
            if main != z { swap(&mut in2out, &mut out2in, &mut swaps, main.as_str(), z); }
            continue;
        }

        let mut root_xor_key = *out2in.get(z).unwrap();
        if root_xor_key.1 != "XOR" || (root_xor_key.0 != main && root_xor_key.2 != main) {
            println!("root XOR is wrong! {z} = {root_xor_key:?}");
            let mut possibilities = Vec::new();
            for (k, v) in in2out.iter() {
                if k.1 == "XOR" && (k.0 == main || k.2 == main) {
                    possibilities.push(*v);
                }
            }
            if possibilities.len() == 0 {
                println!("searching for new main {root_xor_key:?}");
                let _0 = out2in.get(root_xor_key.0).unwrap();
                let _2 = out2in.get(root_xor_key.2).unwrap();
                if (_0.0 == x || _0.0 == y) && (_0.2 == x || _0.2 == y) {
                    swap(&mut in2out, &mut out2in, &mut swaps, main.as_str(), root_xor_key.0);
                }
                else if (_2.0 == x || _2.0 == y) && (_2.2 == x || _2.2 == y) {
                    swap(&mut in2out, &mut out2in, &mut swaps, main.as_str(), root_xor_key.2);
                }
                else {
                    panic!();
                }
                main = get_output(&in2out, main_key).unwrap().to_string();
                for (k, v) in in2out.iter() {
                    if k.1 == "XOR" && (k.0 == main || k.2 == main) {
                        possibilities.push(*v);
                    }
                }
            }
            if root_xor_key.1 != "XOR" || (root_xor_key.0 != main && root_xor_key.2 != main) {
                assert!(possibilities.len() == 1);
                let p = possibilities.pop().unwrap();
                swap(&mut in2out, &mut out2in, &mut swaps, p, z);
                root_xor_key = *out2in.get(z).unwrap();
            }
        }

        assert!(root_xor_key.1 == "XOR");
        assert!(root_xor_key.0 == main || root_xor_key.2 == main);

        let prev_bit = bit - 1;
        let prev_x = format!("x{prev_bit:0>2}");
        let prev_y = format!("y{prev_bit:0>2}");

        let prev_and_key = (prev_x.as_str(), "AND", prev_y.as_str());
        let prev_and = get_output(&in2out, prev_and_key).unwrap().to_string();

        let prev_key = (prev_x.as_str(), "XOR", prev_y.as_str());
        let prev = get_output(&in2out, prev_key).unwrap().to_string();

        if bit == 1 {
            prev_carry = prev_and.to_string();
        }
        else {
            let right_or = if root_xor_key.0 == main { root_xor_key.2 } else { root_xor_key.0 };
            let mut right_or_key = *out2in.get(right_or).unwrap();

            if right_or_key.1 != "OR" || (right_or_key.0 != prev_and && right_or_key.2 != prev_and) {
                println!("right OR is wrong! {right_or} = {right_or_key:?}");
                let mut possibilities = Vec::new();
                for (k, v) in in2out.iter() {
                    if k.1 == "OR" && (k.0 == prev_and || k.2 == prev_and) {
                        possibilities.push(*v);
                    }
                }
                println!("found: {possibilities:?}");
                assert!(possibilities.len() == 1);
                let p = possibilities.pop().unwrap();
                swap(&mut in2out, &mut out2in, &mut swaps, p, right_or);
                right_or_key = *out2in.get(right_or).unwrap();
                println!("new right OR: {right_or} = {right_or_key:?}");
            }
            assert!(right_or_key.1 == "OR");
            assert!(right_or_key.0 == prev_and || right_or_key.2 == prev_and);

            let right_and = if right_or_key.0 == prev_and { right_or_key.2 } else { right_or_key.0 };
            let right_and_key = *out2in.get(right_and).unwrap();

            assert!(right_and_key.1 == "AND");
            assert!(right_and_key.0 == prev || right_and_key.2 == prev);
            assert!(right_and_key.0 == prev_carry || right_and_key.2 == prev_carry);

            prev_carry = right_or.to_string();
            println!("carry: {prev_carry} = {right_or_key:?}");
        }
    }

    swaps.sort();
    let result = swaps.join(",");

    println!("swaps: {result}");

}