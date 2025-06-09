use std::{collections::HashMap, io::Read, usize};


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

    let start_distance_map = build_distance_map(start, &map, usize::MAX, '#');
    let end_distance_map = build_distance_map(end, &map, usize::MAX, '#');

    let base = start_distance_map[&end];

    println!("base: {base}");
    let mut result = 0;
    let mut scores: HashMap<usize, usize> = HashMap::new();

    let cheat_duration = 20;
    let need_to_save = 100;

    for y in 1..map.len() as i32 - 1 {
        for x in 1..map[y as usize].len() as i32 - 1 {
            let cheat_start_pos = (x, y);
            if map[cheat_start_pos.1 as usize][cheat_start_pos.0 as usize] != '.' { continue; }
            let to_cheat_start = start_distance_map.get(&cheat_start_pos);
            if to_cheat_start.is_none() { continue; }

            for dist in 2..cheat_duration+1 {
                for dir in 0..4 {
                    for z in 0..dist {
                        let cheat_end_pos;
                        match dir {
                            0 => { cheat_end_pos = (x + dist - z, y + z); }
                            1 => { cheat_end_pos = (x - z, y + dist - z); }
                            2 => { cheat_end_pos = (x - dist + z, y - z); }
                            3 => { cheat_end_pos = (x + z, y - dist + z); }
                            _ => {unreachable!()}
                        };

                        if
                            cheat_end_pos == cheat_start_pos ||
                            cheat_end_pos.0 < 0 || cheat_end_pos.1 < 0 ||
                            cheat_end_pos.0 >= map[0].len() as i32 ||
                            cheat_end_pos.1 >= map.len() as i32
                        { continue; }
                        let from_cheat_end = end_distance_map.get(&cheat_end_pos);
                        if from_cheat_end.is_none() { continue; }

                        let score = to_cheat_start.unwrap() + from_cheat_end.unwrap() + dist as usize;
                        if score + need_to_save <= base {
                            result += 1;
                            let saved = base - score;
                            *scores.entry(saved).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
    }

    for (saved, count) in scores {
        println!("{count}: {saved}")
    }
    println!("{result}");
}

fn build_distance_map(
    start: (i32, i32),
    map: &Vec<Vec<char>>,
    max_distance: usize,
    stop_char: char
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
    result.insert(pos, distance);

    loop {
        let mut next_poses: Vec<(i32, i32)> = Vec::new();
        for i in 0..4 {
            let next_pos = get_next_pos(pos, i);
            let next_score = distance + 1;
            if
                next_pos.0 < 0 || next_pos.1 < 0 ||
                next_pos.0 >= map[0].len() as i32 || next_pos.1 >= map.len() as i32 ||
                map[next_pos.1 as usize][next_pos.0 as usize] == stop_char ||
                result.get(&next_pos).is_some_and(|s| next_score >= *s)
            {
                continue;
            }
            next_poses.push(next_pos);
        }

        if next_poses.is_empty() || distance >= max_distance {
            if crosses.len() == 0 { break; }

            let cross = crosses.pop().unwrap();
            pos = cross.pos;
            distance = cross.distance;
        }
        else {
            let next_pos = next_poses.pop().unwrap();

            if !next_poses.is_empty() {
                let cross = Cross {distance, pos};
                crosses.push(cross);
            }

            distance += 1;
            pos = next_pos;
            result.insert(pos, distance);
        }
    }

    return result;
}