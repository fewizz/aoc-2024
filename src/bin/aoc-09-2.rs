use std::io::Read;

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut disk: Vec<i32> = Vec::new();

    for (idx, ch) in input.bytes().enumerate() {
        for _ in 0..(ch as i8 - '0' as i8) {
            disk.push(if idx % 2 == 0 { (idx / 2) as i32 } else { -1 });
        }
    }

    let mut right_ptr = disk.len() - 1;

    while right_ptr > 0 {
        let mut right = -1;
        let mut right_len = 0;
        loop {
            let new_right = disk[right_ptr];
            if right == -1 {
                right = new_right;
            }
            else {
                right_len += 1;
                if new_right != right { break; }
            }
            if right_ptr == 0 { break; }
            right_ptr -= 1;
        }

        let mut left_ptr = 0;
        let mut left_len = 0;
        while left_ptr <= right_ptr {
            match disk[left_ptr] {
                -1 => left_len += 1,
                _ => left_len = 0
            };

            left_ptr += 1;

            if left_len >= right_len {
                for i in 0..right_len {
                    disk.swap(right_ptr+1+i, left_ptr-left_len+i);
                }
                break;
            }
        }
    }

    let result = disk.iter()
        .enumerate()
        .fold(0, |result, (idx, id)| {
            result + idx*(if *id != -1 {*id as usize} else {0})
        });
    println!("{result}");
}