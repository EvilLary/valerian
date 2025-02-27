use colors::{BOLD, CYAN, RED, RESET};
use std::fmt;
use std::io;

pub mod args_parser;
pub mod car;
pub mod colors;

pub type ValResult<T> = Result<T, ValError>;

pub enum ValError {
    InvalidArgumnet(&'static str),
    InsufficientArguments(&'static str),
    NotADirectory,
    InvalidOption(String),
    IoError(std::io::Error),
    CurlError(curl::Error),
}

impl From<io::Error> for ValError {
    fn from(error: io::Error) -> Self {
        Self::IoError(error)
    }
}

impl From<curl::Error> for ValError {
    fn from(error: curl::Error) -> Self {
        Self::CurlError(error)
    }
}

impl fmt::Debug for ValError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ValError as E;
        match self {
            E::InvalidArgumnet(o)       =>  write!(f, "{o} \n{CYAN}{BOLD}valerian -h{RESET} for help"),
            E::InsufficientArguments(o) =>  write!(f, "{RED}{BOLD}{o}{RESET}\n{CYAN}{BOLD}valerian -h{RESET} for help"),
            E::NotADirectory            =>  write!(f, "Directory doesn't exist or isn't one\n{CYAN}{BOLD}valerian -h{RESET} for help"),
            E::InvalidOption(o)         =>  write!(f, "Invalid option passed in {RED}{BOLD}{o}{RESET}\n{CYAN}{BOLD}valerian -h{RESET} for help"),
            E::IoError(error)           =>  write!(f, "{}", error.to_string()),
            E::CurlError(error)         =>  write!(f, "{}", error.to_string())
        }
    }
}
