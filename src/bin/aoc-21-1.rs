use std::{collections::HashMap, io::Read, usize};

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut result: usize = 0;

    for line in input.lines() {
        println!("Checking {line}");
        result += complexity(line.chars().collect());
    }
    println!("{result}");
}

fn complexity(moves: Vec<char>) -> usize {
    let directional_keypad = HashMap::from([
        (' ', (0, 0)), ('^', (1, 0)), ('A', (2, 0)),
        ('<', (0, 1)), ('v', (1, 1)), ('>', (2, 1))
    ]);

    let numeric_keypad = HashMap::from([
        ('7', (0, 0)), ('8', (1, 0)), ('9', (2, 0)),
        ('4', (0, 1)), ('5', (1, 1)), ('6', (2, 1)),
        ('1', (0, 2)), ('2', (1, 2)), ('3', (2, 2)),
        (' ', (0, 3)), ('0', (1, 3)), ('A', (2, 3))
    ]);

    fn possible_moves(
        keypad: &HashMap<char, (i32, i32)>,
        moves: &Vec<char>,
        new_moves: Vec<char>,
        idx: usize,
        initial_pos: (i32, i32),
        result: &mut Vec<Vec<char>>
    ) {
        if idx >= moves.len() {
            result.push(new_moves);
            return;
        }

        let next_pos = keypad[&moves[idx]];

        if initial_pos.0 != next_pos.0 && (next_pos.0, initial_pos.1) != keypad[&' '] {
            let mut pos = initial_pos;
            let mut new_moves = new_moves.clone();
            while pos.0 != next_pos.0 {
                new_moves.push(if next_pos.0 < pos.0 {'<'} else {'>'});
                pos.0 += if next_pos.0 < pos.0 {-1} else {1};
            }
            while pos.1 != next_pos.1 {
                new_moves.push(if next_pos.1 < pos.1 {'^'} else {'v'});
                pos.1 += if next_pos.1 < pos.1 {-1} else {1};
            }
            new_moves.push('A');
            possible_moves(keypad, moves, new_moves, idx+1, next_pos, result);
        }
        if initial_pos.1 != next_pos.1 && (initial_pos.0, next_pos.1) != keypad[&' '] {
            let mut pos = initial_pos;
            let mut new_moves = new_moves.clone();
            while pos.1 != next_pos.1 {
                new_moves.push(if next_pos.1 < pos.1 {'^'} else {'v'});
                pos.1 += if next_pos.1 < pos.1 {-1} else {1};
            }
            while pos.0 != next_pos.0 {
                new_moves.push(if next_pos.0 < pos.0 {'<'} else {'>'});
                pos.0 += if next_pos.0 < pos.0 {-1} else {1};
            }
            new_moves.push('A');
            possible_moves(keypad, moves, new_moves, idx+1, next_pos, result);
        }
        if initial_pos == next_pos {
            let mut new_moves = new_moves.clone();
            new_moves.push('A');
            possible_moves(keypad, moves, new_moves, idx+1, next_pos, result);
        }
    }

    let mut result_0 = Vec::new();
    possible_moves(&numeric_keypad, &moves, Vec::new(), 0, numeric_keypad[&'A'], &mut result_0);

    let mut result_1 = Vec::new();
    for r in result_0 {
        possible_moves(&directional_keypad, &r, Vec::new(), 0, directional_keypad[&'A'], &mut result_1);
    }

    let mut result_2 = Vec::new();
    for r in result_1 {
        possible_moves(&directional_keypad, &r, Vec::new(), 0, directional_keypad[&'A'], &mut result_2);
    }

    let initial_moves_parsed = moves[..moves.len()-1].iter().collect::<String>().parse::<usize>().unwrap();
    let min_resulting_moves = result_2.iter().fold(usize::MAX, |o, r| o.min(r.len()));

    return initial_moves_parsed * min_resulting_moves;
}