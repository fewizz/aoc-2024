use std::io::Read;

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut result: i32 = 0;

    'outer: for line in input.lines() {
        let mut prev: Option<i32> = None;
        let mut sign: Option<i32> = None;
        
        for number in line.split(" ").map(|str| str.parse::<i32>().unwrap()) {
            if !check(prev, &mut sign, number) { continue 'outer; }
            prev = Some(number);
        }

        result += 1;
    }

    println!("{result}");
}


fn check(prev: Option<i32>, sign: &mut Option<i32>, number: i32) -> bool {
    match prev {
        Some(p) => {
            let diff = number - p;
            let right_growth = match sign {
                Some(s) => diff.signum() == *s,
                None => { *sign = Some(diff.signum()); true }
            };
            diff.abs() >= 1 && diff.abs() <= 3 && right_growth
        },
        None => true
    }
}