use std::io::Read;

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut iter = input.lines();

    fn read_tuple(line: &str, title: &str) -> (u64, u64) {
        let (x, y) = line[title.len()..].split_once(", ").unwrap();
        (x["X ".len()..].parse::<u64>().unwrap(), y["Y ".len()..].parse::<u64>().unwrap())
    }

    let mut result = 0;

    while let Some(a_str) = iter.next() {
        let (ax, ay) = read_tuple(a_str, "Button A: ");
        let b_str = iter.next().unwrap();
        let (bx, by) = read_tuple(b_str, "Button B: ");
        let p_str = iter.next().unwrap();
        let (px, py) = read_tuple(p_str, "Prize: ");
        let (px, py) = (px+10000000000000, py+10000000000000);

        /*
        ax*a + bx*b = px \
        ay*a + by*b = py /

        =>
        mul first by -ay/ax, then combine
        -ay*bx*b/ax + by*b = -ay*px/ax + py
        b = (py - ay*px/ax) / (by - ay*bx/ax)

        mul first by -by/bx, then combine
        -by*ax*a/bx + ay*a = -by*py/bx + py
        a = (py - by*px/bx) / (ay - by*ax/bx)
        */

        // f64 should be precise enough, right? (:clueless:)

        let (ax_f, ay_f, bx_f, by_f, px_f, py_f) = (
            ax as f64, ay as f64,
            bx as f64, by as f64,
            px as f64, py as f64
        );
        let a_f = (py_f - by_f*px_f/bx_f) / (ay_f - by_f*ax_f/bx_f);
        let b_f = (py_f - ay_f*px_f/ax_f) / (by_f - ay_f*bx_f/ax_f);

        if a_f >= 0.0 && b_f >= 0.0 {
            let a = a_f.round() as u64;
            let b = b_f.round() as u64;

            if 
                ax*a + bx*b == px &&
                ay*a + by*b == py
            {
                result += a*3 + b*1;
            }
        }

        iter.next();  // empty line
    }

    println!("{result}");
}