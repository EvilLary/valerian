use std::env;
use std::path::PathBuf;

pub mod colors;

pub struct CarResponse {
    pub id: String,
    pub url: String,
}
pub struct CmdArgs {
    pub count: u8,
    pub output: PathBuf,
    pub help: bool,
}

pub enum ValResult<T> {
    Ok(T),
    Err(ValError),
}

pub enum ValError {
    InvalidArgumnet,
    LitteArgs,
    ReadOnlyAccess,
    NotADirectory,
    InvalidOption(String),
}

impl CmdArgs {
    pub fn get() -> ValResult<Self> {
        let mut count = 1;
        let mut output = env::current_dir().unwrap();
        let mut help: bool = false;
        let mut args = env::args().skip(1);

        while let Some(item) = args.next() {
            match item.as_str() {
                "-c" => {
                    if let Some(c) = args.next() {
                        match c.parse::<u8>() {
                            Ok(k) => count = k,
                            Err(_e) => {
                                return ValResult::Err(ValError::InvalidArgumnet);
                            }
                        }
                    } else {
                        return ValResult::Err(ValError::LitteArgs);
                    }
                }
                "-o" => {
                    if let Some(o) = args.next() {
                        let path = PathBuf::from(o);

                        if path.is_dir() {
                            // this stupid function is so unreliable it's insane
                            //println!("{:?}",path.metadata().unwrap().permissions().readonly());
                            //
                            //if !path.metadata().unwrap().permissions().readonly() {
                            //    println!("fdf");
                            //    return ValResult::Err(ValError::ReadOnlyAccess);
                            //}
                            output = path;
                        } else {
                            return ValResult::Err(ValError::NotADirectory);
                        }
                    } else {
                        return ValResult::Err(ValError::LitteArgs);
                    }
                }
                "-h" => help = true,
                _ => return ValResult::Err(ValError::InvalidOption(item)),
            }
        }

        ValResult::Ok(Self {
            count,
            output,
            help,
        })
    }
}
