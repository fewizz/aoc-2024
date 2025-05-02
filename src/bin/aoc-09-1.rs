use std::io::Read;

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut disk: Vec<i32> = Vec::new();

    for (mut idx, ch) in input.bytes().enumerate() {
        let is_file = idx % 2 == 0;
        idx /= 2;
        for _ in 0..(ch as i8 - '0' as i8) {
            if is_file {
                disk.push(idx as i32);
            } else {
                disk.push(-1);
            }
        }
    }

    let mut left_idx = 0;
    let mut right_idx = disk.len()-1;

    loop {
        if left_idx >= right_idx { break; }
        while disk[left_idx] != -1 {
            left_idx += 1;
        }
        if left_idx >= right_idx { break; }
        while disk[right_idx] == -1 {
            right_idx -= 1;
        }
        disk.swap(left_idx, right_idx);
    }

    let result = disk.iter()
        .take_while(|x| **x != -1)
        .enumerate()
        .fold(0, |result, (idx, id)| result + idx*(*id as usize));
    println!("{result}");
}