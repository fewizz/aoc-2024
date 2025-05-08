use std::io::Read;

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut iter = input.lines();

    fn read_tuple(line: &str, title: &str) -> (u32, u32) {
        let (x, y) = line[title.len()..].split_once(", ").unwrap();
        (x["X ".len()..].parse::<u32>().unwrap(), y["Y ".len()..].parse::<u32>().unwrap())
    }

    let mut result = 0;

    while let Some(a_str) = iter.next() {
        let a = read_tuple(a_str, "Button A: ");
        let b_str = iter.next().unwrap();
        let b = read_tuple(b_str, "Button B: ");
        let prize_str = iter.next().unwrap();
        let prize = read_tuple(prize_str, "Prize: ");

        let mut min = u32::MAX;
        for a_count in 0..101 {
            for b_count in 0..101 {
                if
                    a_count*3 + b_count*1 < min &&
                    a.0*a_count + b.0*b_count == prize.0 &&
                    a.1*a_count + b.1*b_count == prize.1
                {
                    min = a_count*3 + b_count*1;
                }
            }
        }
        if min != u32::MAX {
            result += min;
        }

        iter.next();  // empty line
    }

    println!("{result}");
}