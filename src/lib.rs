use colors::{BOLD, CYAN, RED, RESET};
use std::fmt;

pub mod api;
pub mod args_parser;
pub mod colors;

pub enum ValError {
    InvalidArgumnet,
    InsufficientArguments(String),
    //ReadOnlyAccess,
    NotADirectory,
    InvalidOption(String),
    IoError(std::io::Error),
    CurlError(curl::Error),
}

impl fmt::Debug for ValError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ValError as E;
        match self {
            E::InvalidArgumnet => write!(f, "Invalid arguments passed in \nsee{CYAN}{BOLD}valerian -h{RESET} for help")?,
            E::InsufficientArguments(o) => {
                write!(f, "{RED}{BOLD}{o}{RESET}\n{CYAN}{BOLD}valerian -h{RESET} for help")?
            }
            E::NotADirectory => write!(f, "Directory doesn't exist or isn't one\n{CYAN}{BOLD}valerian -h{RESET} for help")?,
            E::InvalidOption(o) => write!(f, "Invalid option passed in {RED}{BOLD}{o}{RESET}\n{CYAN}{BOLD}valerian -h{RESET} for help")?,
            E::IoError(error) => write!(f, "{error}")?,
            E::CurlError(error) => write!(f, "{error}")?,
        }
        Ok(())
    }
}
