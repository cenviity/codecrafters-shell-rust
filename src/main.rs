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
            let mut tokens = input.split_whitespace();
            match tokens.next() {
                Some("exit") => process::exit(0),
                Some("echo") => println!("{}", tokens.collect::<Vec<_>>().join(" ")),
                _ => println!("{input}: command not found"),
            }
        }
    }
}
