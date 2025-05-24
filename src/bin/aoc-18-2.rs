use std::{io::Read};


struct Cross {
    score: isize,
    pos: (i32, i32),
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

const SIZE: i32 = 71;


fn main() {

    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut bytes: Vec<(i32, i32)> = Vec::new();

    for line in input.lines() {
        let (x, y) = line.split_once(',').unwrap();
        bytes.push((
            x.parse::<i32>().unwrap(),
            y.parse::<i32>().unwrap()
        ));
    }

    let (mut start, mut end) = (0, bytes.len());

    while start + 1 != end {
        let mid = (start + end) / 2;
        println!("checking {mid}");
        if check_if_reachable(&bytes[..mid]) {
            start = mid;
        }
        else {
            end = mid;
        }
    }

    let (x, y) = bytes[start];
    println!("{x},{y}");
}

fn check_if_reachable(bytes: &[(i32, i32)]) -> bool {
    let mut maze: Vec<Vec<isize>> = (0..SIZE).map(|_| (0..SIZE).map(|_| -1).collect()).collect();
    for b in bytes {
        maze[b.1 as usize][b.0 as usize] = -2;
    }

    let mut pos: (i32, i32) = (0, 0);
    let end: (i32, i32) = (SIZE-1, SIZE-1);

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
    let mut score = 0;

    loop {
        maze[pos.1 as usize][pos.0 as usize] = score;

        let mut next_dirs = Dirs{value: 0};
        for dir in 0..4 {
            let next_pos = get_next_pos(pos, dir);
            if next_pos.0 < 0 || next_pos.1 < 0 || next_pos.0 >= SIZE || next_pos.1 >= SIZE {
                continue;
            }
            let maze_score = maze[next_pos.1 as usize][next_pos.0 as usize];
            if maze_score == -2 || (maze_score >= 0 && maze_score <= score + 1) {
                continue;
            }
            next_dirs.set(dir, true);
        }

        if next_dirs.is_empty() || pos == end {
            if pos == end {
                return true;
            }

            if crosses.len() == 0 {
                break;
            }

            let cross = crosses.pop().unwrap();
            pos = cross.pos;
            score = cross.score;
            continue;
        }

        let next_dir = next_dirs.pop();
        if !next_dirs.is_empty() {
            let cross = Cross {
                score: score,
                pos: pos
            };
            crosses.push(cross);
        }

        pos = get_next_pos(pos, next_dir);
        score += 1;
    }

    return false;
}