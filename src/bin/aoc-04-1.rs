use std::io::Read;

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let table: Vec<Vec<i8>> = input.lines().map(
        |line| line.chars().map(
            |ch| match ch {
                'X' => 0,
                'M' => 1,
                'A' => 2,
                'S' => 3,
                _ => panic!()
            }
        ).collect()
    ).collect();

    let step_patterns: [(i32, i32); 8] = [
        (-1,  1), (0,  1), (1,  1),
        (-1,  0),          (1,  0),
        (-1, -1), (0, -1), (1, -1)
    ];

    let mut result = 0;

    for y in 0..table.len() {
        for x in 0..table[y].len() {
            if table[y][x] != 0 {continue;}

            'pattern_loop: for (step_x, step_y) in step_patterns {
                for distance in 1..4 {
                    let y_pos: usize = match (y as i32 + step_y*distance).try_into() {
                        Result::Ok(v) => v,
                        _ => continue 'pattern_loop
                    };
                    let x_pos: usize = match (x as i32 + step_x*distance).try_into() {
                        Result::Ok(v) => v,
                        _ => continue 'pattern_loop
                    };
                    let table_y = match table.get(y_pos) {
                        Some(t) => t,
                        None => continue 'pattern_loop
                    };
                    let v = match table_y.get(x_pos) {
                        Some(v) => v,
                        None => continue 'pattern_loop
                    };
                    if *v != distance as i8 { continue 'pattern_loop }
                }
                result += 1;
            }
        }
    }

    println!("{result}")
}