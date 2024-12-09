use std::{
    env,
    io::{self, Write},
    path::Path,
    process,
};

pub enum Command<'a> {
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

impl<'a> Command<'a> {
    const BUILTIN_COMMANDS: [&'static str; 4] = ["exit", "echo", "type", "pwd"];

    pub fn parse<'input: 'a>(tokens: Vec<&'input str>) -> Self {
        match tokens[..] {
            ["exit", code] => {
                let code: i32 = code.parse().expect("exit code should be a valid i32 value");
                Self::Exit { code }
            }
            ["echo", ..] => Self::Echo {
                args: tokens[1..].to_owned(),
            },
            ["type", ..] => Self::Type {
                args: tokens[1..].to_owned(),
            },
            ["pwd"] => Self::Pwd,
            ["cd", path] => Self::Cd {
                path: Path::new(path),
            },
            [command, ..] => Self::Other {
                command,
                args: tokens[1..].to_owned(),
            },
            _ => unreachable!(),
        }
    }

    pub fn execute(self) -> io::Result<()> {
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
                continue;
            }

            let Ok(path_env) = env::var("PATH") else {
                eprintln!("Failed to read from `PATH` environment variable");
                return Ok(());
            };

            let mut full_paths = path_env
                .split(":")
                .map(|path_dir| Path::new(path_dir).join(command));
            if let Some(path) = full_paths.find(|path| path.is_file()) {
                println!("{} is {}", command, path.display());
            } else {
                eprintln!("{command}: not found");
            }
        }
        Ok(())
    }

    fn cmd_pwd() -> io::Result<()> {
        let current_dir = env::current_dir()?;
        println!("{}", current_dir.display());
        Ok(())
    }

    fn cmd_cd(path: &Path) -> io::Result<()> {
        if env::set_current_dir(path).is_err() {
            eprintln!("cd: {}: No such file or directory", path.display());
        }
        Ok(())
    }

    fn cmd(command: &str, args: &[&str]) -> io::Result<()> {
        let mut cmd = process::Command::new(command);
        let Ok(output) = cmd.args(args).output() else {
            eprintln!("{command}: command not found");
            return Ok(());
        };
        io::stdout().write_all(&output.stdout)
    }
}

pub fn show_prompt() -> io::Result<()> {
    print!("$ ");
    io::stdout().flush()
}
