//All credit goes to https://github.com/plastic-bottleneck/ccat
//This is just for learning rust

use std::io::Write;
use valerian::{api, args_parser::CmdArgs, ValError};

const HELP_MSG: &str = "Small app to fetch cat pics from TheCatAPI

\x1b[92m\x1b[1mUSAGE:\x1b[0m
    \x1b[96m\x1b[1mvalerian\x1b[0m \x1b[96m[FLAGS] [OPTIONS]\x1b[0m

\x1b[92m\x1b[1mFLAGS:\x1b[0m
   \x1b[1m\x1b[96m-h\x1b[0m        prints help information

\x1b[92m\x1b[1mOPTIONS:\x1b[0m
    \x1b[96m\x1b[1m-c\x1b[0m        number of cats to fetch and download [default: 1]
    \x1b[96m\x1b[1m-o\x1b[0m        output directory [default: current working directory]\n";

#[rustfmt::skip]
fn main() -> Result<(), ValError> {
    let args = CmdArgs::get()?;

    if args.help {
        std::io::stdout()
            .lock()
            .write_all(HELP_MSG.as_bytes())
            .map_err(ValError::IoError)
    } else {
        api::download_cars(
            &api::get_cars(args.count)?,
            &args.output
        )
    }
}
