//All credit goes to https://github.com/plastic-bottleneck/ccat
//This is just for learning rust

use curl::easy::Easy;
use std::{
    env,
    io::{self, Write},
    path::Path,
};

const API_URL: &str = "https://api.thecatapi.com/v1/images/search?mime_types=jpg,png,gif&limit=1";

struct CarResponse {
    id: String,
    url: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    let count: u8 = match args.first() {
        Some(c) => match c.parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("first argument isn't a number defaulting to 1");
                1
            }
        },
        None => 1,
    };
    let save_path: &Path = match args.get(1) {
        Some(o) => {
            let path = Path::new(o);
            if path.is_dir() {
                path
            } else {
                eprint!("{} isn't a directory", path.display());
                std::process::exit(1);
            }
        }
        None => &env::current_dir()?,
    };

    let response: Vec<CarResponse> = get_cars(count)?;
    download_cars(&response, save_path)?;
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
            println!("fetching {i}th car");
            transfer.perform()?;
        }
    }
    Ok(cars)
}

fn download_cars(cars: &Vec<CarResponse>, save_path: &Path) -> Result<(), curl::Error> {
    //TODO make it async?
    let mut handle = Easy::new();
    for car in cars {
        println!("downloading {} from: {}", car.id, car.url);

        let extension = &car.url[car.url.len() - 3..];
        let img_path: String = format!("{}/{}.{}", save_path.display(), car.id, extension);
        let img_file = match std::fs::File::create_new(&img_path) {
            Ok(k) => k,
            Err(e) => {
                eprintln!("Error occured while creating img file\n{e}");
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
                        eprintln!("Error occured while saving image\n{e}");
                    }
                }
                Ok(data.len())
            })?;

            transfer.perform()?;
        }
        //TODO: This seems kinda stupid? it'll print out this message even if image saving wasn't successful
        println!("car saved to {}", img_path);
    }
    Ok(())
}
