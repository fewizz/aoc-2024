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
        'next_ops: for ops in 0..((1 as u64) << ((operands.len()-1)*2)) {
            let mut left = operands[0];
            for i in 1..operands.len() {
                let right = operands[i];
                let op = (ops >> ((i-1)*2)) & 3;
                if op == 0 { left += right; }
                else if op == 1 { left *= right; }
                else if op == 2 {
                    let mut r = right;
                    while r > 0 {
                        r /= 10;
                        left *= 10;
                    }
                    left += right;
                }
                else if op == 3 { continue 'next_ops; } // NOP
                else { panic!(); }
            }
            if left == expected {
                result += left;
                break;
            }
        }
    }

    println!("{result}");
}