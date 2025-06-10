use std::{collections::HashMap, io::Read, usize};

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut result: usize = 0;

    for line in input.lines() {
        // println!("Checking {line}");
        result += complexity(line.chars().collect());
    }
    println!("{result}");
}

fn complexity(moves: Vec<char>) -> usize {

    fn possible_moves(
        prev_key: char,
        next_key: char,
        depth: usize,
        cache: &mut HashMap<(usize, (char, char)), usize>
    ) -> usize {
        if depth == 26 {
            return 1;
        }

        let cached = cache.get(&(depth, (prev_key, next_key)));
        if cached.is_some() {
            return *cached.unwrap();
        }

        let keypad = if depth == 0 {
            HashMap::from([
                ('7', (0, 0)), ('8', (1, 0)), ('9', (2, 0)),
                ('4', (0, 1)), ('5', (1, 1)), ('6', (2, 1)),
                ('1', (0, 2)), ('2', (1, 2)), ('3', (2, 2)),
                (' ', (0, 3)), ('0', (1, 3)), ('A', (2, 3))
            ])
        }
        else {
            HashMap::from([
                (' ', (0, 0)), ('^', (1, 0)), ('A', (2, 0)),
                ('<', (0, 1)), ('v', (1, 1)), ('>', (2, 1))
            ])
        };

        let prev_pos = keypad[&prev_key];
        let next_pos = keypad[&next_key];
        let mut result = usize::MAX;

        if prev_pos.0 != next_pos.0 && (next_pos.0, prev_pos.1) != keypad[&' '] {
            let mut pos = prev_pos;
            let mut local_result = 0;
            let mut prev_key = 'A';
            while pos.0 != next_pos.0 {
                let next_key = if next_pos.0 < pos.0 {'<'} else {'>'};
                local_result += possible_moves(prev_key, next_key, depth+1, cache);
                pos.0 += if next_pos.0 < pos.0 {-1} else {1};
                prev_key = next_key;
            }
            while pos.1 != next_pos.1 {
                let next_key = if next_pos.1 < pos.1 {'^'} else {'v'};
                local_result += possible_moves(prev_key, next_key, depth+1, cache);
                pos.1 += if next_pos.1 < pos.1 {-1} else {1};
                prev_key = next_key;
            }
            local_result += possible_moves(prev_key, 'A', depth+1, cache);
            result = result.min(local_result);
        }
        if prev_pos.1 != next_pos.1 && (prev_pos.0, next_pos.1) != keypad[&' '] {
            let mut pos = prev_pos;
            let mut local_result = 0;
            let mut prev_key = 'A';
            while pos.1 != next_pos.1 {
                let next_key = if next_pos.1 < pos.1 {'^'} else {'v'};
                local_result += possible_moves(prev_key, next_key, depth+1, cache);
                pos.1 += if next_pos.1 < pos.1 {-1} else {1};
                prev_key = next_key;
            }
            while pos.0 != next_pos.0 {
                let next_key = if next_pos.0 < pos.0 {'<'} else {'>'};
                local_result += possible_moves(prev_key, next_key, depth+1, cache);
                pos.0 += if next_pos.0 < pos.0 {-1} else {1};
                prev_key = next_key;
            }
            local_result += possible_moves(prev_key, 'A', depth+1, cache);
            result = result.min(local_result);
        }
        if prev_pos == next_pos {
            let local_result = possible_moves('A', 'A', depth+1, cache);
            result = result.min(local_result);
        }

        if result == usize::MAX {
            panic!();
        }

        cache.insert((depth, (prev_key, next_key)), result);

        return result;
    }

    let initial_moves_parsed = moves[..moves.len()-1].iter().collect::<String>().parse::<usize>().unwrap();
    let mut min_resulting_moves = 0;

    let mut cache: HashMap<(usize, (char, char)), usize> = HashMap::new();
    let mut prev_m = 'A';
    for m in moves {
        min_resulting_moves += possible_moves(prev_m, m, 0, &mut cache);
        prev_m = m;
    }

    return initial_moves_parsed * min_resulting_moves;
}