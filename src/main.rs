use std::io::{self, Write};

use codecrafters_shell::Command;

fn main() -> io::Result<()> {
    loop {
        show_prompt()?;

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        if stdin.read_line(&mut input).is_err() {
            continue;
        }

        let input = input.trim();
        let tokens: Vec<_> = input.split_whitespace().collect();
        let command = Command::parse(tokens);
        command.execute()?
    }
}

fn show_prompt() -> io::Result<()> {
    print!("$ ");
    io::stdout().flush()
}
