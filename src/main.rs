//All credit goes to https://github.com/plastic-bottleneck/ccat
//This is just for learning rust

use curl::easy::Easy;
use serde::Deserialize;
use std::{ffi::OsStr, path::Path};

#[derive(Deserialize)]
struct CarResponse {
    id: String,
    url: String,
}

fn main() -> Result<(), curl::Error> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let count: u8 = match args.get(0) {
        Some(c) => {
            match c.parse() {
                Ok(n) => n,
                Err(_) => {
                    eprintln!("first command isn't number defaulting to 1");
                    1
                }
            }
        }
        None => 1,
    };
    let output: String = match args.get(1) {
        Some(o) => String::from(o),
        None => String::from("."),
    };

    let response: Vec<CarResponse> = get_cars(count)?;
    download_cars(&response, &output)?;
    Ok(())
}

fn get_cars(count: u8) -> Result<Vec<CarResponse>, curl::Error> {
    let mut cars: Vec<CarResponse> = Vec::new();
    if count > 1 {
        cars.reserve_exact(usize::from(count) - 1);
    }

    let url: &str = "https://api.thecatapi.com/v1/images/search?mime_types=jpg,png&limit=1";
    let mut handle = Easy::new();
    handle.url(&url)?;
    {
        let mut transfer = handle.transfer();
        transfer.write_function(|data| {
            let response: Result<Vec<CarResponse>, serde_json::Error> =
                serde_json::from_slice(data);

            match response {
                Ok(car) => {
                    cars.extend(car);
                }
                Err(e) => {
                    eprintln!("Error occured while parsing data\n{e}");
                }
            }

            Ok(data.len())
        })?;

        for i in 1..=count {
            println!("fetching {i}th car");
            transfer.perform()?;
        }
    }
    Ok(cars)
}

fn download_cars(cars: &Vec<CarResponse>, save_path: &str) -> Result<(), curl::Error> {
    //TODO make it async
    for car in cars {
        println!("downloading {} from {}", car.id, car.url);
        if let Some(extension) = Path::new(&car.url).extension().and_then(OsStr::to_str) {
            let mut img = Vec::new();
            let img_path: String = format!("{}/{}.{}", save_path, car.id, extension);

            let mut handle = Easy::new();
            handle.url(&car.url)?;

            {
                let mut transfer = handle.transfer();
                transfer.write_function(|data| {
                    img.extend_from_slice(data);
                    Ok(data.len())
                })?;

                transfer.perform()?;
            }

            match std::fs::write(&img_path, &img) {
                Ok(_) => {
                    println!("car saved to {}", img_path);
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
