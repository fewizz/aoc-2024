use std::io::Read;

// This one was crazy
// Made for *my* input
fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut iter = input.lines();

    iter.next().unwrap();  // skip a
    iter.next().unwrap();  // skip b
    iter.next().unwrap();  // skip c

    iter.next().unwrap();  // empty line

    let prog = iter.next().unwrap()["Program: ".len()..]
        .split(',').map(|c| c.parse::<u8>().unwrap())
        .collect::<Vec<u8>>();

    fn check_a(a_right: isize, level: usize, prog: &Vec<u8>) -> Option<isize> {
        assert!(a_right < (1<<7));

        for a_left in 0..(1 << 3) {
            let a = (a_left << 7) | a_right;
            let mut b = (a % 8) ^ 4;
            b = (b ^ (a >> b)) ^ 4;
            if b % 8 == prog[level] as isize {
                let next_level = level + 1;
                let next_a = a >> 3;

                if next_level == prog.len() && next_a == 0 {
                    return Some(a);
                }
                else if next_level < prog.len() && next_a != 0 {
                    let res = check_a(next_a, next_level, prog);
                    if res.is_some() {
                        return Some((res.unwrap() << 3) | (a & 0b111))
                    }
                }
            }
        }
        return None;
    }

    for initial_a in 0..(1_isize << 7) {
        let res = check_a(initial_a, 0, &prog);
        if res.is_some() {
            let res = res.unwrap();
            println!("{res}");
            return;
        }
    }

    println!("couldn't find such register A");
}