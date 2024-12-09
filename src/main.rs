#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, path::Path, process};

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
    Cd {
        path: &'a Path,
    },
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
            let command = match tokens[..] {
                ["exit", code] => {
                    let code: i32 = code.parse().expect("exit code should be a valid i32 value");
                    Command::Exit { code }
                }
                ["echo", ..] => Command::Echo {
                    args: tokens[1..].to_owned(),
                },
                ["type", ..] => Command::Type {
                    args: tokens[1..].to_owned(),
                },
                ["pwd"] => Command::Pwd,
                ["cd", path] => Command::Cd {
                    path: Path::new(path),
                },
                [command, ..] => Command::Other {
                    command,
                    args: tokens[1..].to_owned(),
                },
                _ => unreachable!(),
            };
            command.execute()?
        }
    }
}

impl Command<'_> {
    const BUILTIN_COMMANDS: [&'static str; 4] = ["exit", "echo", "type", "pwd"];

    fn execute(self) -> io::Result<()> {
        match self {
            Self::Exit { code } => Self::cmd_exit(code),
            Self::Echo { args } => Self::cmd_echo(&args),
            Self::Type { args } => Self::cmd_type(&args),
            Self::Pwd => Self::cmd_pwd(),
            Self::Cd { path } => Self::cmd_cd(path),
            Self::Other { command, args } => Self::cmd(command, &args),
        }
    }

    fn cmd_exit(code: i32) -> io::Result<()> {
        process::exit(code)
    }

    fn cmd_echo(tokens: &[&str]) -> io::Result<()> {
        println!("{}", tokens.join(" "));
        Ok(())
    }

    fn cmd_type(commands: &[&str]) -> io::Result<()> {
        for command in commands {
            if Self::BUILTIN_COMMANDS.contains(command) {
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

    fn cmd_cd(path: &Path) -> io::Result<()> {
        if env::set_current_dir(path).is_err() {
            println!("cd: {}: No such file or directory", path.display());
        }
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
