use crate::{colors::*, ValResult};
use curl::easy::Easy;
use std::{
    fs::File,
    io::{self, Write},
    path::Path,
};

const API_URL: &str = "https://api.thecatapi.com/v1/images/search?mime_types=jpg,png,gif&limit=1";
const BREED_URL: &str = "https://api.thecatapi.com/v1/images/search?breed_ids=";

pub struct Car {
    pub id: String,
    url: String,
}

impl Car {
    pub fn get_cars(count: u8, breed: Option<&str>) -> ValResult<Vec<Self>> {
        let mut cars: Vec<Car> = Vec::with_capacity(usize::from(count));

        let mut handle = Easy::new();
        if let Some(breed_id) = breed {
            let breed_url = format!("{BREED_URL}{breed_id}");
            handle.url(&breed_url)?;
        } else {
            handle.url(API_URL)?;
        }
        {
            let mut transfer = handle.transfer();
            transfer.write_function(|data| {
                if let Some(car) = Car::from_slice(data) {
                    cars.push(car);
                }
                Ok(data.len())
            })?;

            for i in 1..=count {
                println!("{BOLD}{GREEN}INFO{RESET}: fetching {i}th car");
                transfer.perform()?;
            }
        }
        Ok(cars)
    }
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

    pub fn download(&self, save_path: &Path) -> ValResult<()> {
        let img_path = self.find_home(save_path);
        let img_file = File::create_new(&img_path)?;
        let mut img_file = io::BufWriter::new(&img_file);
        let mut handle = Easy::new();
        handle.url(&self.url).unwrap();

        println!(
            "{BOLD}{GREEN}INFO{RESET}: downloading {} from: {BLUE}{}{RESET}",
            self.id, self.url
        );
        {
            let mut transfer = handle.transfer();
            transfer.write_function(|data| {
                if let Err(e) = img_file.write(data) {
                    eprintln!("{BOLD}{GREEN}INFO{RESET}: error saving saving image {e}");
                }
                Ok(data.len())
            })?;
            transfer.perform()?;
        }
        println!("{BOLD}{GREEN}INFO{RESET}: car saved to {BOLD}{GREEN}{img_path}{RESET}");
        Ok(())
    }
}
