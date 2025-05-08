use std::io::Read;


fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut iter = input.lines();
    let mut pos: Option<(i32, i32)> = None;

    let mut map: Vec<Vec<i8>> = Vec::new();
    while let Some(line) = iter.next() {
        if line.is_empty() { break; }

        let mut row: Vec<i8> = Vec::new();
        for ch in line.chars() {
            row.push(match ch {
                '.' => 0, '#' => 1, 'O' => 2,
                '@' => {
                    pos = Some((row.len() as i32, map.len() as i32));
                    0
                },
                _ => panic!()
            });
        }
        map.push(row);
    }

    let mut pos = pos.unwrap();

    fn get_mut_ref(map: &mut Vec<Vec<i8>>, pos: (i32, i32)) -> &mut i8 {
        return map
            .get_mut(pos.1 as usize).unwrap()
            .get_mut(pos.0 as usize).unwrap();
    }

    for line in iter { for ch in line.chars() {
        let step = match ch {
            '>' => ( 1,  0),
            '^' => ( 0, -1),
            '<' => (-1,  0),
            'v' => ( 0,  1),
            _ => panic!()
        };

        let next = (pos.0+step.0, pos.1+step.1);
        if *get_mut_ref(&mut map, next) == 2 {
            let mut after = next;
            loop {
                after.0 += step.0;
                after.1 += step.1;

                let a = get_mut_ref(&mut map, after);
                if a == &0 {
                    *a = 2;
                    *get_mut_ref(&mut map, next) = 0;
                    break;
                }
                else if a == &1 {
                    break;
                }
            }
        }

        if *get_mut_ref(&mut map, next) == 0 {
            pos = next;
        }
    }}

    let result =
        map.iter().enumerate().map(|(y, row)| {
            row.iter().enumerate().map(|(x, v)| {
                if v == &2 {y*100+x} else {0}
            }).reduce(|a, b| a+b).unwrap()
        }).reduce(|a, b| a+b).unwrap();

    println!("{result}");
}