//All credit goes to https://github.com/plastic-bottleneck/ccat
//This is just for learning rust

use valerian::{api, args_parser::CmdArgs, ValError};

#[rustfmt::skip]
fn main() -> Result<(), ValError> {
    let args = CmdArgs::get()?;

    let response = api::get_cars(args.count, args.breed)?;
    api::download_cars(&response, &args.output)
}
