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
            let mut input_words = input.split_whitespace();
            if input_words.next() == Some("exit") {
                process::exit(0);
            } else {
                println!("{input}: command not found");
            }
        }
    }
}
