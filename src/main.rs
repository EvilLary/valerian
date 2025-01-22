//All credit goes to https://github.com/plastic-bottleneck/ccat
//This is just for learning rust
use clap::Parser;
use serde::Deserialize;
use std::{ffi::OsStr, path::Path};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// output directory
    #[arg(short, long, default_value_t = String::from(".") )]
    output: String,

    /// Number of cars to download
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

#[derive(Deserialize, Debug, Clone)]
struct CarResponse {
    id: String,
    url: String,
    //width: u16,
    //height: u16,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    //no need to reference count
    let response: Vec<CarResponse> = get_cars(args.count)?;
    download_cars(&response, &args.output)?;
    Ok(())
}

fn get_cars(count: u8) -> Result<Vec<CarResponse>, reqwest::Error> {
    let url: &str = "https://api.thecatapi.com/v1/images/search?mime_types=jpg,png&limit=1";

    let mut cars: Vec<CarResponse> = Vec::new();
    if count > 1 {
        cars.reserve_exact(usize::from(count) - 1);
    }

    for i in 1..=count {
        println!("fetching {i}th car");
        let response: String = reqwest::blocking::get(url)?.text()?;
        let response: Vec<CarResponse> = match serde_json::from_str(&response) {
            Ok(k) => k,
            Err(e) => {
                eprint!("[Error]: {}", e);
                continue;
                //std::process::exit(1);
            }
        };
        cars.push(response[0].clone())
    }
    Ok(cars)
}

fn download_cars(cars: &Vec<CarResponse>, save_path: &str) -> Result<(), reqwest::Error> {
    //TODO make it async
    for car in cars {
        if let Some(extension) = Path::new(&car.url).extension().and_then(OsStr::to_str) {
            let img_bytes = reqwest::blocking::get(&car.url)?.bytes()?;
            let image = match image::load_from_memory(&img_bytes) {
                Ok(k) => k,
                Err(e) => {
                    eprintln!("Error occured while loading image from memory\n{e}");
                    continue;
                }
            };
            let img_path = format!("{}/{}.{}", save_path, car.id, extension);
            match image.save(&img_path) {
                Ok(_) => {
                    println!("car saved to: {img_path}");
                }
                Err(e) => {
                    eprintln!("Error occured while saving image\n{e}");
                    continue;
                }
            }
        }
    }
    Ok(())
}
