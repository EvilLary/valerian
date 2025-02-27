use crate::colors::*;
use crate::ValError;
use crate::ValResult;
use std::env;
use std::path::PathBuf;

pub struct CmdArgs {
    pub count: u8,
    pub output: PathBuf,
    pub breed: Option<&'static str>,
}

impl CmdArgs {
    pub fn get() -> ValResult<Self> {
        let mut count = None;
        let mut output = env::current_dir()?;
        let mut args = env::args().skip(1);
        let mut breed: Option<&str> = None;

        while let Some(item) = args.next() {
            match item.as_str() {
                "-c" | "--count" => {
                    if let Some(c) = args.next() {
                        count =
                            Some(c.parse::<u8>().map_err(|_| {
                                ValError::InvalidArgumnet("Invalid number provided")
                            })?);
                    } else {
                        return Err(ValError::InsufficientArguments(
                            "-c Must be provided with a number",
                        ));
                    }
                }
                "-o" | "--output" => {
                    if let Some(o) = args.next() {
                        let path = PathBuf::from(o);

                        if path.is_dir() {
                            output = path;
                        } else {
                            return Err(ValError::NotADirectory);
                        }
                    } else {
                        return Err(ValError::InsufficientArguments(
                            "-o must be provided with a directory",
                        ));
                    }
                }
                "-b" | "--breed" => {
                    if let Some(o) = args.next() {
                        if let Some(b) = BREED_LIST.iter().find(|a| *a == &o) {
                            println!("{BOLD}{GREEN}INFO{RESET}: {o} breed selected");
                            breed = Some(b)
                        } else {
                            return Err(ValError::InvalidArgumnet("Invalid breed id"));
                        }
                    } else {
                        return Err(ValError::InsufficientArguments(
                            "-b must be provided a breed id",
                        ));
                    }
                }
                "-bl" | "--breed-list" => {
                    println!("{:?}", BREED_LIST);
                    std::process::exit(0);
                }
                "-h" | "--help" => {
                    println!("{HELP_MSG}");
                    std::process::exit(0);
                }
                _ => return Err(ValError::InvalidOption(item)),
            }
        }
        if let Some(count) = count {
            Ok(Self {
                breed,
                count,
                output,
            })
        } else {
            Err(ValError::InsufficientArguments("counts must be provided"))
        }
    }
}

const BREED_LIST: [&str; 67] = [
    "abys", "aege", "abob", "acur", "asho", "awir", "amau", "amis", "bali", "bamb", "beng", "birm",
    "bomb", "bslo", "bsho", "bure", "buri", "cspa", "ctif", "char", "chau", "chee", "csho", "crex",
    "cymr", "cypr", "drex", "dons", "lihu", "emau", "ebur", "esho", "hbro", "hima", "jbob", "java",
    "khao", "kora", "kuri", "lape", "mcoo", "mala", "manx", "munc", "nebe", "norw", "ocic", "orie",
    "pers", "pixi", "raga", "ragd", "rblu", "sava", "sfol", "srex", "siam", "sibe", "sing", "snow",
    "soma", "sphy", "tonk", "toyg", "tang", "tvan", "ycho",
];

const HELP_MSG: &str = "Small app to fetch cat pics from TheCatAPI

\x1b[92m\x1b[1mUSAGE:\x1b[0m
    \x1b[96m\x1b[1mvalerian\x1b[0m \x1b[96m[FLAGS] [OPTIONS]\x1b[0m

\x1b[92m\x1b[1mFLAGS:\x1b[0m
   \x1b[1m\x1b[96m-h  | --help\x1b[0m               prints help information
   \x1b[1m\x1b[96m-bl | --breed-list\x1b[0m         prints breeds ids

\x1b[92m\x1b[1mOPTIONS:\x1b[0m

    \x1b[96m\x1b[1m-b | --breed\x1b[0m        specify breed to fetch [default: None]
    \x1b[96m\x1b[1m-c | --count\x1b[0m        number of cats to fetch and download [Required]
    \x1b[96m\x1b[1m-o | --output\x1b[0m       output directory [default: current working directory]";
