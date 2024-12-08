#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    path::Path,
    process::{self, Command},
};

const BUILTIN_COMMANDS: [&str; 3] = ["exit", "echo", "type"];

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
            match tokens[..] {
                ["exit", code] => cmd_exit(code),
                ["echo", ..] => cmd_echo(&tokens[1..]),
                ["type", ..] => cmd_type(&tokens[1..]),
                [command, ..] => cmd(command, &tokens[1..])?,
                _ => unreachable!(),
            }
        }
    }
}

fn cmd_exit(code: &str) {
    let code: i32 = code.parse().expect("exit code should be a valid i32 value");
    process::exit(code);
}

fn cmd_echo(tokens: &[&str]) {
    println!("{}", tokens.join(" "));
}

fn cmd_type(commands: &[&str]) {
    for command in commands {
        if BUILTIN_COMMANDS.contains(command) {
            println!("{command} is a shell builtin");
        } else if let Ok(path_env) = std::env::var("PATH") {
            let mut full_paths = path_env
                .split(":")
                .map(|path_dir| Path::new(path_dir).join(command));
            if let Some(path) = full_paths.find(|path| path.is_file()) {
                let path = path
                    .to_str()
                    .expect("full path to command should be valid Unicode");
                println!("{command} is {path}");
            } else {
                println!("{command}: not found");
            }
        }
    }
}

fn cmd(command: &str, args: &[&str]) -> io::Result<()> {
    if let Ok(output) = Command::new(command).args(args).output() {
        io::stdout().write_all(&output.stdout)
    } else {
        println!("{command}: command not found");
        Ok(())
    }
}
