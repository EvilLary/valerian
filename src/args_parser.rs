use crate::ValError;
use std::env;
use std::path::PathBuf;
use crate::colors::*;

pub struct CmdArgs {
    pub count: u8,
    pub output: PathBuf,
    pub breed: Option<String>,
}

impl CmdArgs {
    pub fn get() -> Result<Self, ValError> {
        let mut count = 1;
        let mut output = env::current_dir().map_err(ValError::IoError)?;
        let mut args = env::args().skip(1);
        let mut breed: Option<String> = None;

        while let Some(item) = args.next() {
            match item.as_str() {
                "-c" | "--count" => {
                    if let Some(c) = args.next() {
                        count = c.parse::<u8>().map_err(|_| {
                            ValError::InvalidArgumnet("Invalid number provided".into())
                        })?;
                    } else {
                        return Err(ValError::InsufficientArguments(String::from(
                            "-c Must be provided with a number",
                        )));
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
                        return Err(ValError::InsufficientArguments(String::from(
                            "-o must be provided with a directory",
                        )));
                    }
                }
                "-b" | "--breed" => {
                    if let Some(o) = args.next() {
                        if BREED_LIST.contains(&&o[..]) {
                            println!("{BOLD}{GREEN}INFO{RESET}: {o} breed selected");
                            breed = Some(o)
                        } else {
                            return Err(ValError::InvalidArgumnet("Invalid breed id".into()));
                        }
                    } else {
                        return Err(ValError::InsufficientArguments(
                            "-b must be provided a breed id".into(),
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
        Ok(Self {
            breed,
            count,
            output,
        })
    }
}

pub const BREED_LIST: [&str; 67] = [
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
   \x1b[1m\x1b[96m-bl | --breed-list\x1b[0m        prints breeds ids

\x1b[92m\x1b[1mOPTIONS:\x1b[0m

    \x1b[96m\x1b[1m-b | --breed\x1b[0m        breed id to fetch, see valerian -bl | --breed-list for all breed ids [default: Anything]
    \x1b[96m\x1b[1m-c | --count\x1b[0m        number of cats to fetch and download [default: 1]
    \x1b[96m\x1b[1m-o | --output\x1b[0m       output directory [default: current working directory]\n";
