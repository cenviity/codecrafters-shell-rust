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
            let tokens = tokens(input);
            match tokens[..] {
                ["exit", code] => {
                    let code: i32 = code.parse().expect("exit code should be a valid i32 value");
                    process::exit(code);
                }
                ["echo", ..] => println!("{}", tokens[1..].join(" ")),
                _ => println!("{input}: command not found"),
            }
        }
    }
}

fn tokens(input: &str) -> Vec<&str> {
    input.split_whitespace().collect()
}
