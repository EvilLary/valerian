//All credit goes to https://github.com/plastic-bottleneck/ccat
//This is just for learning rust
use std::thread;
use valerian::{args_parser::CmdArgs, car::CarResponse, ValError};

fn main() -> Result<(), ValError> {
    let args = CmdArgs::get()?;

    let cars = CarResponse::get_cars(args.count, args.breed)?;

    thread::scope(|scope| {
        let save_path = &args.output;
        for car in cars {
            scope.spawn(move || {
                car.download(save_path);
            });
        }
    });
    Ok(())
}
