use core::panic;
use std::io::{self, Write};

fn main() {
    let mut sc = 0;
    loop {
        print!(">");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let chars: Vec<char> = input.chars().collect();
        let a = chars[0] as u32 - 'A' as u32;
        let b = chars[2] as u32 - 'X' as u32;
        let scr = score(a, get_move(a, b));
        sc += scr;
        println!("{scr} {sc}\n");
    }
}
fn get_move(a: u32, oc: u32) -> u32 {
    return match oc {
        0 => (a + 2) % 3,
        1 => a,
        2 => (a + 1) % 3,
        _ => panic!("unknown choice"),
    };
}
fn score(a: u32, b: u32) -> u32 {
    return b
        + 1
        + if a == b {
            3
        } else if b == (a + 1) % 3 {
            6
        } else {
            0
        };
}
