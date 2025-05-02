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

    let mut found_obstacles: HashSet<(isize, isize)> = HashSet::new();
    let mut visited: HashSet<(isize, isize)> = HashSet::new();

    let get_next_pos = |pos: (isize, isize), dir: i32| -> Option<((isize, isize), i8)>{
        let next_pos = match dir {
            0 => (pos.0,   pos.1-1),
            1 => (pos.0+1, pos.1  ),
            2 => (pos.0,   pos.1+1),
            3 => (pos.0-1, pos.1  ),
            _ => panic!()
        };
        if next_pos.0 < 0 || next_pos.1 < 0 ||
           next_pos.0 >= w as isize || next_pos.1 >= (map.len() / w) as isize
        { return None; }
        let v = map[(next_pos.1 as usize * w) + next_pos.0 as usize];
        return Some((next_pos, v));
    };

    loop {
        visited.insert(pos);
        let (next_pos, v) = match get_next_pos(pos, dir) {
            Some(res) => res, None => break
        };

        if v == 1 { dir = (dir + 1) % 4; continue; }
        else if v != 0 { panic!(); }

        // trying to to put an obstacle
        if !visited.contains(&next_pos){
            let mut pos = pos;
            let mut dir = dir;
            let obstacle_pos = next_pos;
            let mut turning_points: Vec<(isize, isize)> = Vec::new();

            'obstacle_check: loop {
                let mut turns = 0;
                loop {
                    let (next_pos, v) = match get_next_pos(pos, dir) {
                        Some(res) => res, None => break 'obstacle_check
                    };

                    if v == 1 || next_pos == obstacle_pos {
                        dir = (dir + 1) % 4;
                        turns += 1;
                    }
                    else if v == 0 {
                        if turns >= 1 {
                            if turning_points.contains(&pos) {
                                found_obstacles.insert(obstacle_pos);
                                break 'obstacle_check;
                            }
                            turning_points.push(pos);
                        }
                        pos = next_pos;
                        break;
                    }
                    else { panic!(); }
                }
            }
        }

        pos = next_pos;
    }

    let result = found_obstacles.len();
    println!("{result}");
}