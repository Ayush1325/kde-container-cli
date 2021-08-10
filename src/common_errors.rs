use std::fmt;

pub enum CommonError {
    StdioParseError(std::string::FromUtf8Error),
    CommandExecuteError(CommandExecuteError),
    IOError(std::io::Error),
    EarlyExit(String),
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
            CommonError::EarlyExit(e) => write!(f, "Exiting:\n{}", e),
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
