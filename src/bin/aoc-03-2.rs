use std::io::Read;

fn main() {
    println!("enter puzzle input:");
    let mut input = String::new();
    std::io::stdin().lock().read_to_string(&mut input).unwrap();

    let mut result: i64 = 0;
    let mut compute = true;
    let mut iter = input.chars();

    'outer: loop {
        let mut ch = match iter.next() { Some(ch) => ch, _ => break };

        if ch == 'd' {
            ch = match iter.next() { Some(ch) => ch, _ => break };
            if ch != 'o' { continue; }
            ch = match iter.next() { Some(ch) => ch, _ => break };
            if ch == '(' {
                ch = match iter.next() { Some(ch) => ch, _ => break };
                if ch != ')' { continue; }
                compute = true;
                continue;
            }
            else if ch == 'n' {
                ch = match iter.next() { Some(ch) => ch, _ => break };
                if ch != '\'' { continue; }
                ch = match iter.next() { Some(ch) => ch, _ => break };
                if ch != 't' { continue; }
                ch = match iter.next() { Some(ch) => ch, _ => break };
                if ch != '(' { continue; }
                ch = match iter.next() { Some(ch) => ch, _ => break };
                if ch != ')' { continue; }
                compute = false; continue;
            }
        }

        if ch != 'm' { continue; }
        ch = match iter.next() { Some(ch) => ch, _ => break };
        if ch != 'u' { continue; }
        ch = match iter.next() { Some(ch) => ch, _ => break };
        if ch != 'l' { continue; }
        ch = match iter.next() { Some(ch) => ch, _ => break };
        if ch != '(' { continue; }

        let mut left = 0;
        
        loop {
            ch = match iter.next() { Some(ch) => ch, _ => break 'outer};
            if ch == ',' { break; }
            if !ch.is_ascii_digit() { continue 'outer; }
            left *= 10; left += ch.to_digit(10).unwrap();
        }

        let mut right = 0;

        loop {
            ch = match iter.next() { Some(ch) => ch, _ => break 'outer};
            if ch == ')' { break; }
            if !ch.is_ascii_digit() { continue 'outer; }
            right *= 10; right += ch.to_digit(10).unwrap();
        }

        if compute {
            result += (left*right) as i64;
        }
    }

    println!("{result}");
}