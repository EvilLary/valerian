use crate::{colors::*, ValError};

const API_URL: &str = "https://api.thecatapi.com/v1/images/search?mime_types=jpg,png,gif&limit=1";
const BREED_URL: &str = "https://api.thecatapi.com/v1/images/search?breed_ids=";

use curl::easy::Easy;
use std::{
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
};

pub struct CarResponse {
    id: String,
    url: String,
}

impl CarResponse {
    fn from_slice(data: &[u8]) -> Option<Self> {
        if let Ok(res) = String::from_utf8(data.to_vec()) {
            //yeah.. idc
            let res = &res[2..res.len() - 2];
            let res: Vec<&str> = res.split(",").collect();

            let id = res[0].split_once(":")?;
            let id = &id.1[1..id.1.len() - 1];
            let id = String::from(id);

            let url = res[1].split_once(":")?;
            let url = &url.1[1..url.1.len() - 1];
            let url = String::from(url);

            return Some(Self { id, url });
        }
        None
    }

    fn find_home(&self, save_path: &Path) -> String {
        let extension = &self.url[self.url.len() - 3..];
        format!("{}/{}.{}", save_path.display(), self.id, extension)
    }
}

pub fn get_cars(count: u8, breed: Option<String>) -> Result<Vec<CarResponse>, ValError> {
    let mut cars: Vec<CarResponse> = Vec::with_capacity(usize::from(count));

    let mut handle = Easy::new();
    if let Some(breed_id) = breed {
        let breed_url = format!("{BREED_URL}{breed_id}");
        handle.url(&breed_url).map_err(ValError::CurlError)?;
    } else {
        handle.url(API_URL).map_err(ValError::CurlError)?;
    }
    {
        let mut transfer = handle.transfer();
        transfer
            .write_function(|data| {
                if let Some(car) = CarResponse::from_slice(data) {
                    cars.push(car);
                }
                Ok(data.len())
            })
            .map_err(ValError::CurlError)?;

        for i in 1..=count {
            println!("{BOLD}{GREEN}INFO{RESET}: fetching {i}th car");
            //println!("{CYAN}fetching {i}th car{RESET}");
            transfer.perform().map_err(ValError::CurlError)?;
        }
    }
    Ok(cars)
}

pub fn download_cars(cars: Vec<CarResponse>, save_path: &Path) -> Result<(), ValError> {
    //use std::sync::Arc

    use std::thread;
    thread::scope(|scope| {
        for car in cars {
            scope.spawn(move || {
                let mut handle = Easy::new();
                println!(
                    "{BOLD}{GREEN}INFO{RESET}: downloading {} from: {BLUE}{}{RESET}",
                    car.id, car.url
                );

                let img_path = car.find_home(save_path);
                let img_file = match File::create_new(&img_path) {
                    Ok(f) => f,
                    Err(e) if e.kind() == io::ErrorKind::PermissionDenied => {
                        panic!("{BOLD}{RED}ERROR{RESET}: No permissions to specified save path")
                    }
                    Err(e) => {
                        eprintln!("{BOLD}{RED}ERROR{RESET}: faild to create image file:  {e}");
                        panic!("fdl");
                    }
                };
                let mut img_file = io::BufWriter::new(&img_file);

                handle.url(&car.url).map_err(ValError::CurlError).unwrap();
                dw_curl(&mut handle, &mut img_file).unwrap();

                println!("{BOLD}{GREEN}INFO{RESET}: car saved to {BOLD}{GREEN}{img_path}{RESET}");
            });
        }
    });
    Ok(())
}

fn dw_curl(handle: &mut Easy, file: &mut BufWriter<&File>) -> Result<(), ValError> {
    let mut transfer = handle.transfer();
    transfer
        .write_function(|data| {
            if let Err(e) = file.write(data) {
                eprintln!("{RED}Error occured while saving image{RESET}\n{e}");
            }
            Ok(data.len())
        })
        .map_err(ValError::CurlError)?;
    transfer.perform().map_err(ValError::CurlError)
}
