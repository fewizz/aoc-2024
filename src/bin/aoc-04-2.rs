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

    let step_patterns = [
        // [( 0,  1), (1,  0), (0, -1), (-1,  0)],
        [(-1,  1), (1,  1), (1, -1), (-1, -1)]
    ];

    let mut result = 0;

    for y in 1..table.len()-1 {
        for x in 1..table[y].len()-1 {
            if table[y][x] != 2 {continue;}
            for step_pattern in step_patterns {
                let mut i = 0;
                let mut prev_m: Option<i8> = None;
                let mut prev_s: Option<i8> = None;
                let mut found_m = false;
                let mut found_s = false;

                for (step_x, step_y) in step_pattern {
                    let y_pos: usize = (y as i32 + step_y) as usize;
                    let x_pos: usize = (x as i32 + step_x) as usize;
                    let v = table[y_pos][x_pos];
                    if v == 1 {
                        if let Some(m) = prev_m {
                            found_m = (m+1)%4 == i || (i+1)%4 == m;
                        }
                        prev_m = Some(i);
                    }
                    if v == 3 {
                        if let Some(s) = prev_s {
                            found_s = (s+1)%4 == i || (i+1)%4 == s;
                        }
                        prev_s = Some(i);
                    }
                    i += 1;
                }
                if found_m && found_s { result += 1; }
            }
        }
    }

    println!("{result}")
}