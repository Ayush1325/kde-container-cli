use core::fmt;
use std::path::Path;

pub trait ContainerOptions {
    fn run(
        &self,
        name: &str,
        attach: bool,
        homepath: &Path,
    ) -> Result<std::process::Child, CommonError>;

    fn build(&self, name: &str) -> Result<std::process::Child, CommonError>;
}

pub enum CommonError {
    StdioParseError(std::string::FromUtf8Error),
    CommandExecuteError(CommandExecuteError),
    IOError(std::io::Error),
}

impl From<std::string::FromUtf8Error> for CommonError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::StdioParseError(e)
    }
}

impl From<std::io::Error> for CommonError {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(e)
    }
}

impl From<CommandExecuteError> for CommonError {
    fn from(e: CommandExecuteError) -> Self {
        Self::CommandExecuteError(e)
    }
}

impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommonError::StdioParseError(e) => write!(f, "Error in parsing STDIO:\n{}", e),
            CommonError::CommandExecuteError(e) => write!(f, "Error in executing command:\n{}", e),
            CommonError::IOError(e) => write!(f, "IO Error:\n{}", e),
        }
    }
}

pub struct CommandExecuteError {
    command: String,
    error: String,
}

impl CommandExecuteError {
    pub fn new(command: String, error: String) -> Self {
        Self { command, error }
    }

    pub fn from_vec(command: String, error: Vec<u8>) -> Self {
        let error = String::from_utf8(error).unwrap_or("Could Not Parse Stderror".to_string());
        Self::new(command, error)
    }
}

impl fmt::Display for CommandExecuteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Command Executed: {}\nStderr Output:{}",
            self.command, self.error
        )
    }
}

pub fn command_to_string(command: &str, args: &[&str]) -> String {
    let combined_args = args.join(" ");
    format!("{} {}", command, combined_args)
}
