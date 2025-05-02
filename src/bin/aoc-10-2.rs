use std::io::Read;

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let map: Vec<Vec<u8>> = input.lines().map(
        |line| line.chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect()
    ).collect();

    let mut result = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] != 0 { continue; }
            let pos = (x as i32, y as i32, 0u8);
            let mut rating: u32 = 0;
            check(&map, pos, &mut rating);
            result += rating;
        }
    }

    println!("{result}");
}

fn check(map: &Vec<Vec<u8>>, pos: (i32, i32, u8), rating: &mut u32) {
    for pattern in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
        let next_pos = (pos.0+pattern.0, pos.1+pattern.1);
        if
            next_pos.0 < 0 || next_pos.1 < 0 ||
            next_pos.0 as usize >= map[0].len() ||
            next_pos.1 as usize >= map.len()
        {
            continue;
        }

        let v = map[next_pos.1 as usize][next_pos.0 as usize];
        if v == pos.2+1 {
            if v == 9 {
                *rating += 1
            }
            else {
                check(map, (next_pos.0, next_pos.1, v), rating);
            }
        }
    }
}