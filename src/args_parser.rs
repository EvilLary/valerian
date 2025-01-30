use crate::ValError;
use std::env;
use std::path::PathBuf;

pub struct CmdArgs {
    pub count: u8,
    pub output: PathBuf,
    pub help: bool,
}

impl CmdArgs {
    pub fn get() -> Result<Self, ValError> {
        let mut count = 1;
        let mut output = env::current_dir().map_err(|e| ValError::IoError(e))?;
        let mut help: bool = false;
        let mut args = env::args().skip(1);

        while let Some(item) = args.next() {
            match item.as_str() {
                "-c" => {
                    if let Some(c) = args.next() {
                        count = c.parse::<u8>().map_err(|_| ValError::InvalidArgumnet)?;
                    } else {
                        return Err(ValError::InsufficientArguments(String::from(
                            "-c Must be provided with a number",
                        )));
                    }
                }
                "-o" => {
                    if let Some(o) = args.next() {
                        let path = PathBuf::from(o);

                        if path.is_dir() {
                            // this stupid function is so unreliable it's insane
                            //if !path.metadata().unwrap().permissions().readonly() {
                            //    println!("fdf");
                            //    return ValResult::Err(ValError::ReadOnlyAccess);
                            //}
                            output = path;
                        } else {
                            return Err(ValError::NotADirectory);
                        }
                    } else {
                        return Err(ValError::InsufficientArguments(String::from(
                            "-o must be provided with a directory",
                        )));
                    }
                }
                "-h" => help = true,
                _ => return Err(ValError::InvalidOption(item)),
            }
        }
        Ok(Self {
            count,
            output,
            help,
        })
    }
}
