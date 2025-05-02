use std::io::Read;

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut result: u64 = 0;

    for line in input.lines() {
        let mut iter = line.split(" ");
        let r = iter.next().unwrap();
        let expected = r[..r.len()-1].parse::<u64>().unwrap();

        let mut operands: Vec<u64> = Vec::new();
        while let Some(v) = iter.next() {
            operands.push(v.parse::<u64>().unwrap());
        }

        //let mut ops: u64 = 0;
        for ops in 0..(1<<(operands.len()-1)) {
            let mut left = operands[0];
            for i in 1..operands.len() {
                let right = operands[i];
                let op = (ops >> (i-1)) & 1;
                if op == 0 { left += right; }
                if op == 1 { left *= right; }
            }
            if left == expected {
                result += left;
                break;
            }
        }
    }

    println!("{result}");
}