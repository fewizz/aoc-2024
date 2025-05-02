use std::{collections::HashSet, io::Read};

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut map: Vec<i8> = Vec::new();
    let mut w = 0; 
    let mut dir = 0;
    let mut pos: (isize, isize) = (-1, -1);

    for (y, line) in input.lines().enumerate() {
        w = line.len();
        for (x, ch) in line.chars().enumerate() {
            if ch == '.' { map.push(0) }
            else if ch == '#' { map.push(1) }
            else if ch == '^' { pos = (x as isize, y as isize); map.push(0);}
            else { panic!(); }
        }
    }

    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    loop {
        visited.insert(pos);

        let next_pos = match dir {
            0 => (pos.0,   pos.1-1),
            1 => (pos.0+1, pos.1  ),
            2 => (pos.0,   pos.1+1),
            3 => (pos.0-1, pos.1  ),
            _ => panic!()
        };
        if next_pos.0 < 0 || next_pos.1 < 0 ||
           next_pos.0 > w as isize || next_pos.1 > (map.len() / w) as isize
        { break; }

        let next_idx = (next_pos.1 as usize * w) + next_pos.0 as usize;
        let v = map[next_idx];

        if v == 0 { pos = next_pos; }
        else if v == 1 { dir = (dir + 1) % 4; }
        else { panic!(); }
    }

    let result = visited.len();
    println!("{result}");
}