use std::env;
use std::path::PathBuf;

pub const API_URL: &str =
    "https://api.thecatapi.com/v1/images/search?mime_types=jpg,png,gif&limit=1";
pub const HELP_MSG: &str = r#"Small app to fetch cats pic from TheCatAPI

    USAGE:
        valerian [FLAGS] [OPTIONS]

    [FLAGS]:
        -h        prints help information
        -v        prints version information
        -V        enable verbose output

    [OPTIONS]:
        -c        number of cats to fetch and download [default: 1]
        -o        output directory [default: current working directory]"#;

pub struct CmdArgs {
    pub count: u8,
    pub output: PathBuf,
    pub verbose: bool,
    //pub version: bool,
    pub help: bool,
}

impl CmdArgs {
    pub fn get() -> Self {
        let mut count = 1;
        let mut output = env::current_dir().unwrap();
        let mut verbose: bool = false;
        let mut help: bool = false;
        //let mut version: bool = false;
        let mut args = env::args().skip(1);

        while let Some(item) = args.next() {
            if item == "-c" {
                if let Some(c) = args.next() {
                    match c.parse::<u8>() {
                        Ok(k) => count = k,
                        Err(_e) => {
                            eprintln!("Invalid value passed to -c");
                        }
                    }
                }
            } else if item == "-o" {
                if let Some(o) = args.next() {
                    let path = PathBuf::from(o);

                    if path.is_dir() {
                        if path.metadata().unwrap().permissions().readonly() {
                            eprint!("Don't have write access to {}", path.display());
                            std::process::exit(1);
                        }
                        output = path;
                    } else {
                        eprint!("{} isn't a directory", path.display());
                        std::process::exit(1);
                    }
                }
            } else if item == "-h" {
                help = true;
            } else if item == "-V" {
                verbose = true;
            } else {
                eprintln!("Invalid argument");
            }
        }

        Self {
            count,
            output,
            verbose,
            help,
        }
    }
}
//pub fn parse_args() -> (u8, PathBuf) {
//    let args = env::args().skip(1).collect::<Vec<String>>();
//    let count: u8 = match args.first() {
//        Some(c) => {
//            if let Ok(n) = c.parse() {
//                n
//            } else {
//                eprint!("First argument has to be a number");
//                std::process::exit(1);
//            }
//        }
//        None => 1,
//    };
//
//    let save_path: PathBuf = match args.get(1) {
//        Some(o) => {
//            let path = PathBuf::from(o);
//
//            if path.is_dir() {
//                if path.metadata().unwrap().permissions().readonly() {
//                    eprint!("Don't have write access to {}", path.display());
//                    std::process::exit(1);
//                }
//                path
//            } else {
//                eprint!("{} isn't a directory", path.display());
//                std::process::exit(1);
//            }
//        }
//        None => env::current_dir().expect("Failed to retrieve current directory"),
//    };
//
//    (count, save_path)
//}
