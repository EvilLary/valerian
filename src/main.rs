//All credit goes to https://github.com/plastic-bottleneck/ccat
//This is just for learning rust

use std::io::Write;
use valerian::{
    api,
    colors::{CYAN, GREEN, RESET},
    args_parser::CmdArgs,
    ValError,
};

fn main() -> Result<(), ValError> {
    let args = CmdArgs::get()?;
    if args.help {
        help_msg()?;
    } else {
        let response = api::get_cars(args.count)?;
        api::download_cars(&response, &args.output)?;
    }
    Ok(())
}

fn help_msg() -> Result<(), ValError> {
    let msg = format!(
        "Small app to fetch cats pic from TheCatAPI

{GREEN}USAGE:{RESET}
    valerian {CYAN}[FLAGS] [OPTIONS]{RESET}

{GREEN}[FLAGS]:{RESET}
    {CYAN}-h        prints help information{RESET}

{GREEN}[OPTIONS]:{RESET}
    {CYAN}-c        number of cats to fetch and download [default: 1]
    -o        output directory [default: current working directory]{RESET}"
    );

    std::io::stdout()
        .lock()
        .write_all(format!("{}\n", msg).as_bytes())
        .map_err(|e| ValError::IoError(e))?;
    Ok(())
}
