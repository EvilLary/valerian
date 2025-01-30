//All credit goes to https://github.com/plastic-bottleneck/ccat
//This is just for learning rust

use colors::*;
use curl::easy::Easy;
use std::{
    io::{self, Write},
    path::PathBuf,
    process::exit,
};
use valerian::*;


const API_URL: &str =
    "https://api.thecatapi.com/v1/images/search?mime_types=jpg,png,gif&limit=1";

fn main() -> Result<(), curl::Error> {
    let args = match CmdArgs::get() {
        ValResult::Ok(k) => k,
        ValResult::Err(e) => {
            match e {
                ValError::NotADirectory => {
                    eprintln!("Provided directory isn't one or it doesn't exists");
                }
                ValError::InvalidArgumnet => {
                    eprintln!("Invalid argumnet passed in, {BOLD}see valerian -h for help{RESET} ");
                }
                ValError::ReadOnlyAccess => {
                    eprintln!(
                        "Insuffucient permissions to the provided directory {RED}Read Only{RESET}"
                    );
                }
                ValError::InvalidOption(n) => {
                    eprintln!(
                        "Invalid option {:?}\nsee {BOLD}valerian -h{RESET} for options",
                        n
                    )
                }
                ValError::LitteArgs => {
                    eprintln!("No argumnet provided to option");
                }
            }
            exit(1);
        }
    };

    if args.help {
        //println!("{HELP_MSG}");
        help_msg();
        return Ok(());
    }
    let response = get_cars(args.count)?;

    if let Err(e) = download_cars(&response, &args.output) {
        eprintln!("{RED}Error occured while downloading cars{RESET}: {e}");
        std::process::exit(1);
    }

    Ok(())
}

fn get_cars(count: u8) -> Result<Vec<CarResponse>, curl::Error> {
    let mut cars: Vec<CarResponse> = Vec::new();
    if count > 1 {
        cars.reserve_exact(usize::from(count) - 1);
    }

    let mut handle = Easy::new();
    handle.url(API_URL)?;
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|data| {
            if let Ok(res) = String::from_utf8(data.to_vec()) {
                //yeah.. idc
                let res = &res[2..res.len() - 2];
                let res: Vec<&str> = res.split(",").collect();

                let id = res[0].split_once(":").unwrap();
                let id = &id.1[1..id.1.len() - 1];
                let id = String::from(id);

                let url = res[1].split_once(":").unwrap();
                let url = &url.1[1..url.1.len() - 1];
                let url = String::from(url);

                cars.push(CarResponse { id, url });
            }
            Ok(data.len())
        })?;

        for i in 1..=count {
            println!("{CYAN}fetching {i}th car{RESET}");
            transfer.perform()?;
        }
    }
    Ok(cars)
}

fn download_cars(cars: &[CarResponse], save_path: &PathBuf) -> Result<(), curl::Error> {
    //TODO make it async?
    let mut handle = Easy::new();
    for car in cars {
        println!(
            "downloading {} from: {BOLD}{BLUE}{}{RESET}",
            car.id, car.url
        );

        let extension = &car.url[car.url.len() - 3..];
        let img_path: String = format!("{}/{}.{}", save_path.display(), car.id, extension);
        let img_file = match std::fs::File::create_new(&img_path) {
            Ok(k) => k,
            Err(e) => {
                eprintln!("{RED}Error occured while creating img file{RESET}\n{e}");
                continue;
            }
        };
        //TODO: maybe use with_capcity ?
        let mut img_file = io::BufWriter::new(&img_file);

        handle.url(&car.url)?;
        {
            let mut transfer = handle.transfer();
            transfer.write_function(|data| {
                match img_file.write(data) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("{RED}Error occured while saving image{RESET}\n{e}");
                    }
                }
                Ok(data.len())
            })?;

            transfer.perform()?;
        }
        //TODO: This seems kinda stupid? it'll print out this message even if image saving wasn't successful
        println!("{BLUE}car saved to {BOLD}{GREEN}{}{RESET}", img_path);
    }
    Ok(())
}

fn help_msg() {

    let msg = format!("Small app to fetch cats pic from TheCatAPI

{GREEN}USAGE:{RESET}
    valerian {CYAN}[FLAGS] [OPTIONS]{RESET}

{GREEN}[FLAGS]:{RESET}
    {CYAN}-h        prints help information{RESET}

{GREEN}[OPTIONS]:{RESET}
    {CYAN}-c        number of cats to fetch and download [default: 1]
    -o        output directory [default: current working directory]{RESET}");

    //std::io::stdout()
    //    .lock()
    //    .write_all(format!("{}\n", msg).as_bytes())
    //    .expect("Failed to write to stdout");
    println!("{msg}");
}
