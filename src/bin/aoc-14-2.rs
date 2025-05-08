use std::io::Read;

// First try!

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let (width, height) = (101, 103);

    let mut robots: Vec<((i32, i32), (i32, i32))> = Vec::new();

    for line in input.lines() {
        let (p, v) = line.split_once(" ").unwrap();
        let p = p["p=".len()..].split_once(",").unwrap();
        let v = v["v=".len()..].split_once(",").unwrap();
        let p = (p.0.parse::<i32>().unwrap(), p.1.parse::<i32>().unwrap());
        let v = (v.0.parse::<i32>().unwrap(), v.1.parse::<i32>().unwrap());

        robots.push((p, v));
    }

    let mut result = 0;
    let mut max_near_count = 0;
    let mut max_near_robots = robots.clone();

    for s in 1..10000 {
        for r in robots.iter_mut() {
            r.0.0 = (r.0.0 + r.1.0).rem_euclid(width);
            r.0.1 = (r.0.1 + r.1.1).rem_euclid(height);
        }
        let mut near_count = 0;
        for cur_i in 0..robots.len() {
            for other_i in cur_i+1..robots.len() {
                let cur = robots[cur_i];
                let other = robots[other_i];

                let x_diff = cur.0.0.abs_diff(other.0.0);
                let y_diff = cur.0.1.abs_diff(other.0.1);

                if (x_diff >= 1 && x_diff <= 2) && (y_diff >= 1 && y_diff <= 2) {
                    near_count += 1;
                }
            }
        }
        if near_count > max_near_count {
            max_near_count = near_count;
            result = s;
            max_near_robots = robots.clone();
        }
    }

    // I WANT to look at it
    for y in 0..height {
        for x in 0..width {
            if max_near_robots.iter().any(|r| r.0.0 == x && r.0.1 == y) {
                print!("*");
            }
            else {
                print!(" ");
            }
        }
        println!("");
    }

    println!("{result}");
}