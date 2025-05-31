use std::{collections::HashMap, io::Read};


fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut start: Option<(i32, i32)> = None;
    let mut end: Option<(i32, i32)> = None;

    let map: Vec<Vec<char>> =
        input.lines().enumerate().map(|(y, l)| {
        l.chars().enumerate().map(|(x, c)| {
            if c == 'S' {
                start = Some((x as i32, y as i32));
                return '.'
            }
            if c == 'E' {
                end = Some((x as i32, y as i32));
                return '.'
            }
            c
        }).collect()
    }).collect();

    let start = start.unwrap();
    let end = end.unwrap();

    let start_distance_map = build_distance_map(start, &map);
    let end_distance_map = build_distance_map(end, &map);

    let base = start_distance_map[&end];

    println!("base: {base}");
    let mut result = 0;

    for y in 1..map.len() as i32 - 1 {
        for x in 1..map[y as usize].len() as i32 - 1 {
            let cheat_start_pos = (x, y);
            if map[cheat_start_pos.1 as usize][cheat_start_pos.0 as usize] != '.' { continue; }
            for step in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                let wall_pos = (x+step.0*1, y+step.1*1);
                if map[wall_pos.1 as usize][wall_pos.0 as usize] != '#' { continue; }
                let cheat_end_pos = (x+step.0*2, y+step.1*2);
                if
                    cheat_end_pos.0 < 0 || cheat_end_pos.1 < 0 ||
                    cheat_end_pos.0 >= map[0].len() as i32 ||
                    cheat_end_pos.1 >= map.len() as i32
                { continue; }

                let to_cheat_start = start_distance_map.get(&cheat_start_pos);
                let from_cheat_end = end_distance_map.get(&cheat_end_pos);
                if to_cheat_start.is_none() || from_cheat_end.is_none() { continue; }

                let score = to_cheat_start.unwrap() + from_cheat_end.unwrap() + 2;
                if score + 100 <= base { result += 1; }
            }
        }
    }

    println!("{result}");
}

fn build_distance_map(
    start: (i32, i32),
    map: &Vec<Vec<char>>
) -> HashMap<(i32, i32), usize> {
    let mut pos = start;

    struct Cross {
        distance: usize,
        pos: (i32, i32)
    }

    fn get_next_pos(pos: (i32, i32), dir: u8) -> (i32, i32) {
        match dir {
            0 => (pos.0+1, pos.1+0),
            1 => (pos.0+0, pos.1-1),
            2 => (pos.0-1, pos.1+0),
            3 => (pos.0+0, pos.1+1),
            _ => panic!()
        }
    }

    let mut crosses: Vec<Cross> = Vec::new();
    let mut result: HashMap<(i32, i32), usize> = HashMap::new();
    let mut distance = 0;

    loop {
        result.insert(pos, distance);

        let mut next_poses: Vec<(i32, i32)> = Vec::new();
        for i in 0..4 {
            let next_pos = get_next_pos(pos, i);
            let next_score = distance + 1;
            if
                map[next_pos.1 as usize][next_pos.0 as usize] == '#' ||
                result.get(&next_pos).is_some_and(|s| next_score >= *s)
            {
                continue;
            }
            next_poses.push(next_pos);
        }

        if next_poses.is_empty() {
            if crosses.len() == 0 {
                break;
            }

            let cross = crosses.pop().unwrap();
            pos = cross.pos;
            distance = cross.distance;
        }
        else {
            let next_pos = next_poses.pop().unwrap();

            if !next_poses.is_empty() {
                let cross = Cross {distance, pos: pos};
                crosses.push(cross);
            }

            distance += 1;
            pos = next_pos;
        }
    }

    return result;
}