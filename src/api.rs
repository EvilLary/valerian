use crate::{colors::*, ValError};

const API_URL: &str = "https://api.thecatapi.com/v1/images/search?mime_types=jpg,png,gif&limit=1";

use curl::easy::Easy;
use std::{
    io::{self, Write},
    path::Path,
};

pub struct CarResponse {
    id: String,
    url: String,
}

pub fn get_cars(count: u8) -> Result<Vec<CarResponse>, ValError> {
    let mut cars: Vec<CarResponse> = Vec::new();
    if count > 1 {
        cars.reserve_exact(usize::from(count) - 1);
    }

    let mut handle = Easy::new();
    handle.url(API_URL).map_err(ValError::CurlError)?;
    {
        let mut transfer = handle.transfer();
        transfer
            .write_function(|data| {
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
            })
            .map_err(ValError::CurlError)?;

        let mut stdout = std::io::stdout().lock();
        for i in 1..=count {
            writeln!(stdout, "{CYAN}fetching {i}th car{RESET}").map_err(ValError::IoError)?;
            transfer.perform().map_err(ValError::CurlError)?;
        }
    }
    Ok(cars)
}

pub fn download_cars(cars: &[CarResponse], save_path: &Path) -> Result<(), ValError> {
    //TODO make it async?
    let mut handle = Easy::new();
    let mut stdout = std::io::stdout().lock();
    for car in cars {
        writeln!(
            stdout,
            "downloading {} from: {BOLD}{BLUE}{}{RESET}",
            car.id, car.url
        )
        .map_err(ValError::IoError)?;

        let extension = &car.url[car.url.len() - 3..];
        let img_path: String = format!("{}/{}.{}", save_path.display(), car.id, extension);
        let img_file = std::fs::File::create_new(&img_path).map_err(ValError::IoError)?;

        //TODO: maybe use with_capcity ?
        let mut img_file = io::BufWriter::new(&img_file);

        handle.url(&car.url).map_err(ValError::CurlError)?;
        {
            let mut transfer = handle.transfer();
            transfer
                .write_function(|data| {
                    match img_file.write(data) {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("{RED}Error occured while saving image{RESET}\n{e}");
                        }
                    }
                    Ok(data.len())
                })
                .map_err(ValError::CurlError)?;

            transfer.perform().map_err(ValError::CurlError)?;
        }
        //TODO: This seems kinda stupid? it'll print out this message even if image saving wasn't successful
        writeln!(stdout, "{BLUE}car saved to {BOLD}{GREEN}{img_path}{RESET}")
            .map_err(ValError::IoError)?;
    }
    Ok(())
}
