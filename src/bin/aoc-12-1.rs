use std::{collections::HashMap, io::Read};

#[derive(Clone, Copy)]
struct Plot {
    area: u32,
    perimeter: u32
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
        for (x, char) in line.bytes().enumerate() {
            let upper_plot_id = row[x];
            let left_plot_id = if x > 0 { row[x-1] } else { (0, 0) };

            if left_plot_id.0 == char && upper_plot_id.0 == char {
                let upper_plot = plots[&upper_plot_id].clone();
                let left_plot = plots.get_mut(&left_plot_id).unwrap();

                if left_plot_id == upper_plot_id {
                    left_plot.area += 1;
                    *row.get_mut(x).unwrap() = left_plot_id;
                }
                else {
                    for z in 0..row.len() {
                        if row[z] == upper_plot_id {
                            *row.get_mut(z).unwrap() = left_plot_id;
                        }
                    }
                    left_plot.area += upper_plot.area + 1;
                    left_plot.perimeter += upper_plot.perimeter;
                    plots.remove(&upper_plot_id);
                }
            }
            else if left_plot_id.0 == char {
                let left_plot = plots.get_mut(&left_plot_id).unwrap();
                left_plot.area += 1;
                left_plot.perimeter += 2;
                *row.get_mut(x).unwrap() = left_plot_id;
            }
            else if upper_plot_id.0 == char {
                let upper_plot = plots.get_mut(&upper_plot_id).unwrap();
                upper_plot.area += 1;
                upper_plot.perimeter += 2;
                *row.get_mut(x).unwrap() = upper_plot_id;
            }
            else {
                plots.insert((char, next_id), Plot{area: 1, perimeter: 4});
                *row.get_mut(x).unwrap() = (char, next_id);
                next_id += 1;
            }
        }
    }

    let result = plots.values().fold(0, |left, right| left + right.area * right.perimeter);
    println!("{result}");
}