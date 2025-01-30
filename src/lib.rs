pub mod api;
pub mod colors;
pub mod args_parser;

#[derive(Debug)]
pub enum ValError {
    InvalidArgumnet,
    InsufficientArguments(String),
    //ReadOnlyAccess,
    NotADirectory,
    InvalidOption(String),
    IoError(std::io::Error),
    CurlError(curl::Error),
}
