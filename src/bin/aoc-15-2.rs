use std::io::Read;

// I don't like my solution, dirty

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut iter = input.lines();
    let mut pos: Option<(i32, i32)> = None;

    let mut map: Vec<Vec<char>> = Vec::new();
    while let Some(line) = iter.next() {
        if line.is_empty() { break; }

        let mut row: Vec<char> = Vec::new();
        for ch in line.chars() {
            let v = match ch {
                '.' => ('.', '.'), '#' => ('#', '#'), 'O' => ('[', ']'),
                '@' => {
                    pos = Some((row.len() as i32, map.len() as i32));
                    ('.', '.')
                },
                _ => panic!()
            };
            row.push(v.0);
            row.push(v.1);
        }
        map.push(row);
    }

    let mut pos = pos.unwrap();

    fn get_mut_ref(map: &mut Vec<Vec<char>>, pos: (i32, i32)) -> &mut char {
        return map
            .get_mut(pos.1 as usize).unwrap()
            .get_mut(pos.0 as usize).unwrap();
    }

    fn get(map: &Vec<Vec<char>>, pos: (i32, i32)) -> char {
        return *map
            .get(pos.1 as usize).unwrap()
            .get(pos.0 as usize).unwrap();
    }

    for line in iter { for ch in line.chars() {
        let step = match ch {
            '>' => ( 1,  0),
            '^' => ( 0, -1),
            '<' => (-1,  0),
            'v' => ( 0,  1),
            _ => panic!()
        };

        let next_pos = (pos.0+step.0, pos.1+step.1);
        let next = get(&map, next_pos);
        if next == '#' {}
        else if next == '.' {
            pos = next_pos;
        }
        else {
            if step.0 != 0 {
                let mut free_pos = next_pos;
                loop {
                    free_pos.0 += step.0;
                    match get(&map, free_pos) {
                        '.' => {
                            break;
                        }
                        '#' => {
                            free_pos = (-1, -1);
                            break;
                        }
                        _ => { continue; }
                    }
                }
                if free_pos != (-1, -1) {
                    while free_pos != next_pos {
                        let last_box_pos = (free_pos.0-step.0, pos.1);
                        *get_mut_ref(&mut map, free_pos) = get(&map, last_box_pos);
                        *get_mut_ref(&mut map, last_box_pos) = '.';
                        free_pos = last_box_pos;
                    }
                    pos = next_pos;
                }
            }
            else {
                let mut to_move: Vec<Vec<(i32, i32)>> = Vec::new();

                to_move.push(Vec::new());
                to_move.last_mut().unwrap().push(next_pos);

                'outer: loop {
                    let ps = to_move.last_mut().unwrap();
                    for i in 0..ps.len() {
                        let p = ps[i];
                        let v = get(&map, p);
                        if v == '[' && !ps.contains(&(p.0+1, p.1)) {
                            ps.push((p.0+1, p.1));
                        }
                        if v == ']' && !ps.contains(&(p.0-1, p.1)) {
                            ps.push((p.0-1, p.1));
                        }
                    }

                    let mut next_to_move: Vec<(i32, i32)> = Vec::new();

                    for p in ps {
                        let next_pos = (p.0, p.1+step.1);
                        let next = get(&map, next_pos);
                        if next == '.' {}
                        else if next == '#' {
                            to_move.clear();
                            break 'outer;
                        }
                        else if next == '[' || next == ']' {
                            next_to_move.push(next_pos);
                        }
                    }
                    if next_to_move.len() > 0 {
                        to_move.push(next_to_move);
                        continue;
                    }

                    // ok, now we can move everything
                    for p in to_move.iter().rev().flatten() {
                        *get_mut_ref(&mut map, (p.0, p.1+step.1)) = get(&map, *p);
                        *get_mut_ref(&mut map, *p) = '.';
                    }
                    pos = next_pos;
                    break;
                }
            }
        }

        /*println!("{ch}:");
        for (y, row) in map.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                let mut v = v;
                if pos.0 == x as i32 && pos.1 == y as i32 {
                    v = &'@';
                }
                print!("{v}");
            }
            println!()
        }*/
    }}

    let result =
        map.iter().enumerate().map(|(y, row)| {
            row.iter().enumerate().map(|(x, v)| {
                if v == &'[' {y*100+x} else {0}
            }).reduce(|a, b| a+b).unwrap()
        }).reduce(|a, b| a+b).unwrap();

    println!("{result}");
}