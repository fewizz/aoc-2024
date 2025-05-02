use std::io::Read;

#[derive(PartialEq)]
enum Stage {
    M, U, L, OB, Left, Right
}

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut stage = Stage::M;
    let mut left = 0;
    let mut right = 0;

    let mut result: i64 = 0;

    for ch in input.chars() {
        if stage == Stage::M && ch == 'm' { stage = Stage::U; continue; }
        if stage == Stage::U && ch == 'u' { stage = Stage::L; continue; }
        if stage == Stage::L && ch == 'l' { stage = Stage::OB; continue; }
        if stage == Stage::OB && ch == '(' { stage = Stage::Left; continue; }
        if stage == Stage::Left && ch.is_ascii_digit() { left *= 10; left += ch.to_digit(10).unwrap(); continue; }
        if stage == Stage::Left && ch == ',' { stage = Stage::Right; continue; }
        if stage == Stage::Right && ch.is_ascii_digit() { right *= 10; right += ch.to_digit(10).unwrap(); continue; }
        if stage == Stage::Right && ch == ')' { result += (left*right) as i64; }
        left = 0;
        right = 0;
        stage = Stage::M;
    }

    println!("{result}");
}