use std::{
    env,
    io::{self, Write},
    path::{Path, PathBuf},
    process,
};

pub enum Command<'a> {
    Exit(i32),
    Echo(Vec<&'a str>),
    Type(Vec<&'a str>),
    Pwd,
    Cd(PathType),
    Other {
        command: &'a str,
        args: Vec<&'a str>,
    },
}

pub enum PathType {
    HomeDir,
    OtherPath(PathBuf),
}

impl PathType {
    fn parse(path: &str) -> Self {
        match path {
            "~" => Self::HomeDir,
            _ => Self::OtherPath(PathBuf::from(path)),
        }
    }
}

impl From<PathType> for PathBuf {
    fn from(path_type: PathType) -> Self {
        match path_type {
            PathType::HomeDir => {
                Self::from(env::var("HOME").expect("$HOME environment variable should exist"))
            }
            PathType::OtherPath(path) => path,
        }
    }
}

impl<'a> Command<'a> {
    const BUILTIN_COMMANDS: [&'static str; 5] = ["exit", "echo", "type", "pwd", "cd"];

    pub fn parse<'input: 'a>(tokens: Vec<&'input str>) -> Self {
        match tokens[..] {
            ["exit", code] => {
                Self::Exit(code.parse().expect("exit code should be a valid i32 value"))
            }
            ["echo", ..] => Self::Echo(tokens[1..].to_owned()),
            ["type", ..] => Self::Type(tokens[1..].to_owned()),
            ["pwd"] => Self::Pwd,
            ["cd", path] => Self::Cd(PathType::parse(path)),
            [command, ..] => Self::Other {
                command,
                args: tokens[1..].to_owned(),
            },
            _ => unreachable!(),
        }
    }

    pub fn execute(self) -> io::Result<()> {
        match self {
            Self::Exit(code) => Self::cmd_exit(code),
            Self::Echo(args) => Self::cmd_echo(&args),
            Self::Type(args) => Self::cmd_type(&args),
            Self::Pwd => Self::cmd_pwd(),
            Self::Cd(path_type) => Self::cmd_cd(path_type),
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
                eprintln!("Failed to read from $PATH environment variable");
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

    fn cmd_cd(path_type: PathType) -> io::Result<()> {
        let path = PathBuf::from(path_type);
        if env::set_current_dir(&path).is_err() {
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
