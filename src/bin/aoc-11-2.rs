use std::{collections::HashMap, io::Read};

fn main() {
    // Ah, good old "just throw in some cache" :)
    // Probably can be solved much simpler, can't see how tho

    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    fn stone(blink: u32, num: u64, cache: &mut HashMap<(u32, u64), u64>) -> u64 {
        if let Some(cached) = cache.get(&(blink, num)) {
            return *cached;
        }

        if blink == 75 { return 1; }

        let result;

        if num == 0 {
            result = stone(blink+1, 1, cache);
        }
        else {
            let mut cpy = num;
            let mut half_mul = 1;
            let mut digits_count = 0;

            while cpy > 0 {
                digits_count += 1;
                if digits_count % 2 == 0 { half_mul *= 10; }
                cpy /= 10;
            }

            if digits_count % 2 == 0 {
                result =
                    stone(blink+1, num / half_mul, cache) +
                    stone(blink+1, num % half_mul, cache);
            }
            else {
                result = stone(blink+1, num * 2024, cache);
            }
        }

        cache.insert((blink, num), result);
        return result;
    }

    let mut result = 0;
    let mut cache: HashMap<(u32, u64), u64> = HashMap::new();
    for num in input.split_whitespace().map(
        |num_str| num_str.parse::<u64>().unwrap()
    ) {
        result += stone(0, num, &mut cache);
    }

    println!("{result}");
}