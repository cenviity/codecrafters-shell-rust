#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    path::Path,
    process::{self},
};

const BUILTIN_COMMANDS: [&str; 4] = ["exit", "echo", "type", "pwd"];

enum Command<'a> {
    Exit {
        code: i32,
    },
    Echo {
        args: Vec<&'a str>,
    },
    Type {
        args: Vec<&'a str>,
    },
    Pwd,
    Other {
        command: &'a str,
        args: Vec<&'a str>,
    },
}

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
                ["exit", code] => Command::cmd_exit(code),
                ["echo", ..] => Command::cmd_echo(&tokens[1..])?,
                ["type", ..] => Command::cmd_type(&tokens[1..])?,
                ["pwd"] => Command::cmd_pwd()?,
                [command, ..] => Command::cmd(command, &tokens[1..])?,
                _ => unreachable!(),
            }
        }
    }
}

impl Command<'_> {
    fn cmd_exit(code: &str) {
        let code: i32 = code.parse().expect("exit code should be a valid i32 value");
        process::exit(code);
    }

    fn cmd_echo(tokens: &[&str]) -> io::Result<()> {
        println!("{}", tokens.join(" "));
        Ok(())
    }

    fn cmd_type(commands: &[&str]) -> io::Result<()> {
        for command in commands {
            if BUILTIN_COMMANDS.contains(command) {
                println!("{command} is a shell builtin");
            } else if let Ok(path_env) = std::env::var("PATH") {
                let mut full_paths = path_env
                    .split(":")
                    .map(|path_dir| Path::new(path_dir).join(command));
                if let Some(path) = full_paths.find(|path| path.is_file()) {
                    println!("{} is {}", command, path.display());
                } else {
                    println!("{command}: not found");
                }
            }
        }
        Ok(())
    }

    fn cmd_pwd() -> io::Result<()> {
        let current_dir = std::env::current_dir()?;
        println!("{}", current_dir.display());
        Ok(())
    }

    fn cmd(command: &str, args: &[&str]) -> io::Result<()> {
        if let Ok(output) = process::Command::new(command).args(args).output() {
            io::stdout().write_all(&output.stdout)
        } else {
            println!("{command}: command not found");
            Ok(())
        }
    }
}
