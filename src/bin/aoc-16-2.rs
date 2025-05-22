// Too bloated, but anyway...

use std::{collections::{HashMap, HashSet}, io::Read};


struct Cross {
    score: usize,
    path_idx: usize,
    dir: u8,
    dirs_to_check: Dirs
}

struct Dirs {
    value: u8
}

impl Dirs {
    fn set(&mut self, dir: u8, v: bool) {
        if v {
            self.value |= 1 << dir;
        }
        else {
            self.value &= !(1 << dir);
        }
    }

    fn is_empty(&self) -> bool {
        self.value == 0
    }

    fn get(&self, idx: u8) -> bool {
        return (self.value >> idx) & 1 == 1;
    }

    fn pop(&mut self) -> u8 {
        for i in 0..4 {
            if self.get(i) {
                self.set(i, false);
                return i;
            }
        }
        panic!();
    }
}


fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut pos: Option<(i32, i32)> = None;
    let mut end: Option<(i32, i32)> = None;

    let maze: Vec<Vec<char>> =
        input.lines().enumerate().map(|(y, l)| {
        l.chars().enumerate().map(|(x, c)| {
            if c == 'S' {
                pos = Some((x as i32, y as i32));
                return '.'
            }
            if c == 'E' {
                end = Some((x as i32, y as i32));
                return '.'
            }
            c
        }).collect()
    }).collect();

    let mut pos = pos.unwrap();
    let end = end.unwrap();
    let mut dir = 0;

    fn get_next_pos(pos: (i32, i32), dir: u8) -> (i32, i32) {
        match dir {
            0 => (pos.0+1, pos.1+0),
            1 => (pos.0+0, pos.1-1),
            2 => (pos.0-1, pos.1+0),
            3 => (pos.0+0, pos.1+1),
            _ => panic!()
        }
    }

    fn get_next_score(score: usize, dir: u8, next_dir: u8) -> usize {
        score + 1 + match next_dir.abs_diff(dir) {
            0 => 0,
            1 => 1000,
            2 => panic!(),
            3 => 1000,
            _ => panic!()
        }
    }

    let mut crosses: Vec<Cross> = Vec::new();
    let mut path: Vec<(i32, i32)> = Vec::new();
    let mut scores: HashMap<(i32, i32, u8), usize> = HashMap::new();
    let mut score = 0;

    let mut best_spots: HashSet<(i32, i32)> = HashSet::new();
    let mut result = usize::MAX;

    // let mut i = 0;

    loop {
        path.push(pos);

        /*i += 1;
        if i % 1 == 0 {
            let mut data: Vec<u8> = Vec::new();
            for (y, row) in maze.iter().enumerate() {
                for (x, c) in row.iter().enumerate() {
                    let mut c = *c;
                    if best_spots.contains(&(x as i32, y as i32)){
                        c = 'v';
                    }
                    data.push(c as u8);
                }
                data.push('\n' as u8);
            }
            std::io::stdout().write_all(&data).unwrap();
        }*/

        let mut next_dirs = Dirs{value: 0};
        for i in -1..1+1 {
            let next_dir = (dir as i8 + i).rem_euclid(4) as u8;
            let next_pos = get_next_pos(pos, next_dir);
            let next_score = get_next_score(score, dir, next_dir);
            if
                maze[next_pos.1 as usize][next_pos.0 as usize] == '#' ||
                scores.get(
                    &(next_pos.0, next_pos.1, next_dir))
                    .is_some_and(|s| next_score > *s  // not >= as before
                )
            {
                continue;
            }
            next_dirs.set(next_dir, true);
        }

        let next_dir: u8;

        if next_dirs.is_empty() || pos == end {
            if pos == end {
                if score < result {
                    result = score;
                    best_spots.clear();
                }

                if score == result {
                    best_spots.extend(path.iter());
                }
            }

            while crosses.pop_if(|c|c.dirs_to_check.is_empty()).is_some() {
            }

            if crosses.len() == 0 {
                break;
            }

            let cross = crosses.last_mut().unwrap();
            dir = cross.dir;
            pos = path[cross.path_idx];
            path.truncate(cross.path_idx+1);
            score = cross.score;
            next_dir = cross.dirs_to_check.pop();
        }
        else {
            scores.insert((pos.0, pos.1, dir), score);
            next_dir = next_dirs.pop();

            if !next_dirs.is_empty() {
                let cross = Cross {
                    score: score,
                    path_idx: path.len()-1,
                    dir: dir,
                    dirs_to_check: next_dirs
                };
                crosses.push(cross);
            }
        }

        score = get_next_score(score, dir, next_dir);
        pos = get_next_pos(pos, next_dir);
        dir = next_dir;
    }

    let result = best_spots.len();
    println!("{result}");
}