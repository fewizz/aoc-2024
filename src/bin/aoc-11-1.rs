use std::io::Read;

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut list: Vec<u64> = input.split_whitespace().map(
        |num_str| num_str.parse::<u64>().unwrap())
    .collect();

    for blink in 1..76 {
        println!("blink {blink}");
        let mut next_list: Vec<u64> = Vec::new();

        for num in list {
            if num == 0 { next_list.push(1); continue; }

            let mut cpy = num;
            let mut half_mul = 1;
            let mut digits_count = 0;

            while cpy > 0 {
                digits_count += 1;
                if digits_count % 2 == 0 { half_mul *= 10; }
                cpy /= 10;
            }

            if digits_count % 2 == 0 {
                next_list.push(num / half_mul);
                next_list.push(num % half_mul);
            }
            else {
                next_list.push(num * 2024);
            }
        }

        list = next_list;
    }

    let result = list.len();
    println!("{result}");
}