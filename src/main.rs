#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        if stdin.read_line(&mut input).is_ok() {
            println!("{}: command not found", input.trim());
        }
    }
}
