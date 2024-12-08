#[allow(unused_imports)]
use std::io::{self, Write};
use std::process;

fn main() -> io::Result<()> {
    loop {
        print!("$ ");
        io::stdout().flush()?;

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();

        if stdin.read_line(&mut input).is_ok() {
            let input = input.trim();
            let tokens: Vec<_> = input.split_whitespace().collect();
            match tokens[0] {
                "exit" => process::exit(0),
                "echo" => println!("{}", tokens[1..].join(" ")),
                _ => println!("{input}: command not found"),
            }
        }
    }
}
