use std::io::Read;


fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut iter = input.lines();

    let mut a_reg = iter.next().unwrap()["Register A: ".len()..].parse::<usize>().unwrap();
    let mut b_reg = iter.next().unwrap()["Register B: ".len()..].parse::<usize>().unwrap();
    let mut c_reg = iter.next().unwrap()["Register C: ".len()..].parse::<usize>().unwrap();

    iter.next().unwrap();  // empty line

    let prog = iter.next().unwrap()["Program: ".len()..]
        .split(',').map(|c| c.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let mut ip: usize = 0;
    let mut output: Vec<u8> = Vec::new();

    while ip < prog.len() {
        let opcode = prog[ip];
        let operand = prog[ip+1];

        ip += 2;

        let combo = || {
            if operand <= 3 { return operand }
            if operand == 4 { return a_reg }
            if operand == 5 { return b_reg }
            if operand == 6 { return c_reg }
            else { unreachable!() }
        };

        match opcode {
            0 => {  // adv
                a_reg >>= combo();
            },
            1 => {  // bxl
                b_reg ^= operand;
            },
            2 => {  // bst
                b_reg = combo() % 8;
            },
            3 => {  // jnz
                if a_reg > 0 {
                    ip = operand as usize;
                }
            },
            4 => {  // bxc
                b_reg ^= c_reg;
            },
            5 => {  // out
                output.push((combo() % 8) as u8);
            },
            6 => {  // bdv
                b_reg = a_reg >> combo();
            },
            7 => {  // cdv
                c_reg = a_reg >> combo();
            }
            _ => { unreachable!() }
        };
    }

    for (idx, ch) in output.iter().enumerate() {
        print!("{ch}");
        if idx != output.len()-1 {
            print!(",");
        }
    }
}