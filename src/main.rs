//All credit goes to https://github.com/plastic-bottleneck/ccat
//This is just for learning rust
use std::thread;
use valerian::{args_parser::CmdArgs, car::Car, colors::*, ValResult};

fn main() -> ValResult<()> {
    let args = CmdArgs::get()?;
    let cars = Car::get_cars(args.count, args.breed)?;

    thread::scope(|scope| {
        let save_path = &args.output;
        cars.iter().for_each(|car| {
            scope.spawn(move || {
                if let Err(e) = car.download(save_path) {
                    println!(
                        "{BOLD}{RED}ERROR{RESET}: error downloading {}, {:?}",
                        car.id, e
                    );
                }
            });
        });
    });
    Ok(())
}
