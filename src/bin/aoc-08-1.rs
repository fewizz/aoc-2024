use std::{collections::{HashMap, HashSet}, io::Read};

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut freqs: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();
    let mut map: Vec<Vec<u8>> = Vec::new();

    for (y, line) in input.lines().enumerate() {
        map.push(line.bytes().enumerate().map(|(x, ch)| {
            if ch != '.' as u8 {
                freqs.entry(ch).or_insert_with(Vec::new)
                    .push((x as i32, y as i32));
            }
            ch
        }).collect());
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for positions in freqs.values() {
        for pos_a in positions.iter() {
            for pos_b in positions.iter().filter(|p| *p != pos_a) {
                let pos_c = (
                    pos_b.0 + (pos_b.0 - pos_a.0),
                    pos_b.1 + (pos_b.1 - pos_a.1)
                );
                if
                    pos_c.0 < 0 || pos_c.1 < 0 ||
                    pos_c.0 as usize >= map[0].len() ||
                    pos_c.1 as usize >= map.len()
                {
                    continue;
                }

                // let v = map[pos_c.1 as usize][pos_c.0 as usize];
                // if v == '.' as u8 {
                    antinodes.insert(pos_c);
                // }
            }
        }
    }

    let result = antinodes.len();
    println!("{result}");
}