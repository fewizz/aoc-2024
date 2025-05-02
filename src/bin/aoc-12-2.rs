use std::{collections::HashMap, io::Read};

#[derive(Clone, Copy)]
struct Plot {
    area: u32,
    sides: u32
}

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut plots: HashMap<(u8, u32), Plot> = HashMap::new();
    let mut next_id: u32 = 0;

    let mut row: Vec<(u8, u32)> = vec![
        (0, 0);
        input.lines().next().unwrap().len()
    ];

    for line in input.lines() {
        let mut upper_left_plot_id = (0, 0);

        for (x, char) in line.bytes().enumerate() {
            let mut upper_plot_id = row[x];
            let left_plot_id = if x > 0 { row[x-1] } else { (0, 0) };

            if left_plot_id.0 == char && upper_plot_id.0 == char {
                let upper_plot = plots[&upper_plot_id].clone();
                let left_plot = plots.get_mut(&left_plot_id).unwrap();

                if left_plot_id == upper_plot_id {
                    left_plot.area += 1;
                    if x+1 >= row.len() || row[x+1] != left_plot_id {
                        left_plot.sides -= 2;
                    }
                    *row.get_mut(x).unwrap() = left_plot_id;
                }
                else {
                    for z in 0..row.len() {
                        if row[z] == upper_plot_id {
                            *row.get_mut(z).unwrap() = left_plot_id;
                        }
                    }
                    left_plot.area += upper_plot.area + 1;
                    left_plot.sides += upper_plot.sides;
                    if x+1 >= row.len() || row[x+1] != left_plot_id {
                        left_plot.sides -= 2;
                    }
                    plots.remove(&upper_plot_id);
                    upper_plot_id = left_plot_id;
                }
            }
            else if left_plot_id.0 == char {
                let left_plot = plots.get_mut(&left_plot_id).unwrap();
                left_plot.area += 1;
                if upper_left_plot_id == left_plot_id {
                    left_plot.sides += 2; // bump to the right
                }
                *row.get_mut(x).unwrap() = left_plot_id;
            }
            else if upper_plot_id.0 == char {
                let upper_plot = plots.get_mut(&upper_plot_id).unwrap();
                upper_plot.area += 1;
                let upper_left = upper_left_plot_id == upper_plot_id;
                let upper_right = x+1 < row.len() && row[x+1] == upper_plot_id;
                if upper_left || upper_right {
                    upper_plot.sides += if upper_left && upper_right {4} else {2}; // bump to the bottom
                }
                *row.get_mut(x).unwrap() = upper_plot_id;
            }
            else {
                plots.insert((char, next_id), Plot{area: 1, sides: 4});
                *row.get_mut(x).unwrap() = (char, next_id);
                next_id += 1;
            }

            upper_left_plot_id = upper_plot_id;
        }
    }

    let result = plots.values().fold(0, |left, right| left + right.area * right.sides);
    println!("{result}");
}