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


fn main() {
    let size = 71;
    let mut bytes_count = 1024;

    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut maze: Vec<Vec<isize>> = (0..size).map(|_| (0..size).map(|_| -1).collect()).collect();

    for line in input.lines() {
        let (x, y) = line.split_once(',').unwrap();
        maze[y.parse::<usize>().unwrap()][x.parse::<usize>().unwrap()] = -2;
        bytes_count -= 1;
        if bytes_count == 0 {
            break;
        }
    }

    let mut pos: (i32, i32) = (0, 0);
    let end: (i32, i32) = (size-1, size-1);

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
    let mut result = isize::MAX;
    let mut i = 0;

    loop {
        i += 1;
        if i % 1000000 == 0 {
            for y in 0..size {
                for x in 0..size {
                    let v = maze[y as usize][x as usize];
                    if v == -2 {
                        print!("#");
                    }
                    else if v == -1 {
                        print!(" ");
                    }
                    else {
                        print!("v");
                    }
                }
                println!();
            }
            println!();
        }

        maze[pos.1 as usize][pos.0 as usize] = score;

        let mut next_dirs = Dirs{value: 0};
        for dir in 0..4 {
            let next_pos = get_next_pos(pos, dir);
            if next_pos.0 < 0 || next_pos.1 < 0 || next_pos.0 >= size || next_pos.1 >= size {
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
                result = result.min(score);
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

    println!("{result}");
}