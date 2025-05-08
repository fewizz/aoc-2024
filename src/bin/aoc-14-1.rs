use std::io::Read;

// First try!

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let (width, height) = (101, 103);
    let (center_x, center_y) = (width / 2, height / 2);

    let mut counts = [0, 0, 0, 0];

    for line in input.lines() {
        let (p, v) = line.split_once(" ").unwrap();
        let p = p["p=".len()..].split_once(",").unwrap();
        let v = v["v=".len()..].split_once(",").unwrap();
        let p = (p.0.parse::<i32>().unwrap(), p.1.parse::<i32>().unwrap());
        let v = (v.0.parse::<i32>().unwrap(), v.1.parse::<i32>().unwrap());

        let final_p = (
            (p.0 + v.0*100).rem_euclid(width),
            (p.1 + v.1*100).rem_euclid(height)
        );

        if final_p.0 == center_x || final_p.1 == center_y {
            continue;
        }

        counts[
           (if final_p.0 < center_x {0} else {1}) * 2 + 
            if final_p.1 < center_y {0} else {1}
        ] += 1;
    }

    let result = counts.iter().fold(1, |i, c| i*c);
    println!("{result}");
}