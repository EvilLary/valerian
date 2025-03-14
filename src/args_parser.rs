use crate::colors::*;
use crate::ValError;
use crate::ValResult;
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;

pub struct CmdArgs {
    pub count: u8,
    pub output: PathBuf,
    pub breed: Option<&'static str>,
}

impl CmdArgs {
    pub fn get() -> ValResult<Self> {
        let mut count = None;
        let mut output = env::current_dir()?;
        let mut args = env::args().skip(1);
        let mut breed: Option<&str> = None;

        while let Some(item) = args.next() {
            match item.as_str() {
                "-c" | "--count" => {
                    if let Some(c) = args.next() {
                        count =
                            Some(c.parse::<u8>().map_err(|_| {
                                ValError::InvalidArgumnet("Invalid number provided")
                            })?);
                    } else {
                        return Err(ValError::InsufficientArguments(
                            "-c Must be provided with a number",
                        ));
                    }
                }
                "-o" | "--output" => {
                    if let Some(o) = args.next() {
                        let path = PathBuf::from(o);

                        if path.is_dir() {
                            output = path;
                        } else {
                            return Err(ValError::NotADirectory);
                        }
                    } else {
                        return Err(ValError::InsufficientArguments(
                            "-o must be provided with a directory",
                        ));
                    }
                }
                "-b" | "--breed" => {
                    if let Some(o) = args.next() {
                        if let Some(b) = BREED_LIST.iter().find(|a| *a == &o) {
                            println!("{BOLD}{GREEN}INFO{RESET}: {o} breed selected");
                            breed = Some(b)
                        } else {
                            return Err(ValError::InvalidArgumnet("Invalid breed id"));
                        }
                    } else {
                        return Err(ValError::InsufficientArguments(
                            "-b must be provided a breed id",
                        ));
                    }
                }
                "-bl" | "--breed-list" => {
                    breed_list()?;
                    std::process::exit(0);
                }
                "-h" | "--help" => {
                    println!("{HELP_MSG}");
                    std::process::exit(0);
                }
                _ => return Err(ValError::InvalidOption(item)),
            }
        }
        if let Some(count) = count {
            Ok(Self {
                breed,
                count,
                output,
            })
        } else {
            Err(ValError::InsufficientArguments("counts must be provided"))
        }
    }
}

fn breed_list() -> io::Result<()> {
    let mut stdout = io::stdout().lock();
    writeln!(
        stdout,
        "╔ Breed List ════════════════════╦══════════════════════════════════════════════════════╗"
    )?;
    writeln!(
        stdout,
        "║   ID  ║         NAME           ║                  WIKI URL                            ║"
    )?;
    writeln!(
        stdout,
        "╠═══════╬════════════════════════╬══════════════════════════════════════════════════════╣"
    )?;
    for (i, (id, (name, url))) in BREED_LIST.iter().zip(BREED_INFO).enumerate() {
        // 20: numbers of characters in the larget name
        // 51: numbers of characters in the larget url + 1
        // This is not well designed but This what I've got to get nice formatting
        let nm_whs = 20 - name.len();
        let url_whs = 51 - url.len();
        let color = if i % 2 == 0 { GREEN } else { BLUE };
        writeln!(
            stdout,
            "╠═{color} {id} {RESET}╠═{color} {name}  {RESET}{}╠═{color} {url}{RESET}{} ║",
            " ".repeat(nm_whs),
            " ".repeat(url_whs)
        )?;
    }
    writeln!(
        stdout,
        "╚═══════╩════════════════════════╩══════════════════════════════════════════════════════╝"
    )?;
    Ok(())
}

#[rustfmt::skip]
const BREED_INFO: [(&str, &str); 67] = [
    ("Abyssinian", "https://en.wikipedia.org/wiki/Abyssinian_cat"),
    ("Aegean", "https://en.wikipedia.org/wiki/Aegean_cat"),
    ("American Bobtail", "https://en.wikipedia.org/wiki/American_Bobtail"),
    ("American Curl", "https://en.wikipedia.org/wiki/American_Curl"),
    ("American Shorthair", "https://en.wikipedia.org/wiki/American_Shorthair"),
    ("American Wirehair", "https://en.wikipedia.org/wiki/American_Wirehair"),
    ("Arabian Mau", "https://en.wikipedia.org/wiki/Arabian_Mau"),
    ("Australian Mist", "https://en.wikipedia.org/wiki/Australian_Mist"),
    ("Balinese", "https://en.wikipedia.org/wiki/Balinese_(cat)"),
    ("Bambino", "https://en.wikipedia.org/wiki/Bambino_cat"),
    ("Bengal", "https://en.wikipedia.org/wiki/Bengal_(cat)"),
    ("Birman", "https://en.wikipedia.org/wiki/Birman"),
    ("Bombay", "https://en.wikipedia.org/wiki/Bombay_(cat)"),
    ("British Longhair", "https://en.wikipedia.org/wiki/British_Longhair"),
    ("British Shorthair", "https://en.wikipedia.org/wiki/British_Shorthair"),
    ("Burmese", "https://en.wikipedia.org/wiki/Burmese_(cat)"),
    ("Burmilla", "https://en.wikipedia.org/wiki/Burmilla"),
    ("California Spangled", "https://en.wikipedia.org/wiki/California_Spangled"),
    ("Chantilly-Tiffany", "https://en.wikipedia.org/wiki/Chantilly-Tiffany"),
    ("Chartreux", "https://en.wikipedia.org/wiki/Chartreux"),
    ("Chausie", "https://en.wikipedia.org/wiki/Chausie"),
    ("Chettoh", "https://en.wikipedia.org/wiki/Bengal_cat#Cheetoh"),
    ("Colorpoint Shorthair", "https://en.wikipedia.org/wiki/Colorpoint_Shorthair"),
    ("Cornish Rex", "https://en.wikipedia.org/wiki/Cornish_Rex"),
    ("Cymric", "https://en.wikipedia.org/wiki/Cymric_(cat)"),
    ("Cyprus", "https://en.wikipedia.org/wiki/Cyprus_cat"),
    ("Devon Rex", "https://en.wikipedia.org/wiki/Devon_Rex"),
    ("Donskoy", "https://en.wikipedia.org/wiki/Donskoy_(cat)"),
    ("Dragon Li", "https://en.wikipedia.org/wiki/Dragon_Li"),
    ("Egyptian Mau", "https://en.wikipedia.org/wiki/Egyptian_Mau"),
    ("European Burmese", "https://en.wikipedia.org/wiki/Burmese_cat"),
    ("Exotic Shorthair", "https://en.wikipedia.org/wiki/Exotic_Shorthair"),
    ("Havana Brown", "https://en.wikipedia.org/wiki/Havana_Brown"),
    ("Himalayan", "https://en.wikipedia.org/wiki/Himalayan_(cat)"),
    ("Japanese Bobtail", "https://en.wikipedia.org/wiki/Japanese_Bobtail"),
    ("Javanese", "https://en.wikipedia.org/wiki/Javanese_cat"),
    ("Khao Manee", "https://en.wikipedia.org/wiki/Khao_Manee"),
    ("Korat", "https://en.wikipedia.org/wiki/Korat"),
    ("Kurilian", "https://en.wikipedia.org/wiki/Kurilian_Bobtail"),
    ("LaPerm", "https://en.wikipedia.org/wiki/LaPerm"),
    ("Maine Coon", "https://en.wikipedia.org/wiki/Maine_Coon"),
    ("Malayan", "https://en.wikipedia.org/wiki/Asian_cat"),
    ("Manx", "https://en.wikipedia.org/wiki/Manx_(cat)"),
    ("Munchkin", "https://en.wikipedia.org/wiki/Munchkin_(cat)"),
    ("Nebelung", "https://en.wikipedia.org/wiki/Nebelung"),
    ("Norwegian Forest Cat", "https://en.wikipedia.org/wiki/Norwegian_Forest_Cat"),
    ("Ocicat", "https://en.wikipedia.org/wiki/Ocicat"),
    ("Oriental", "https://en.wikipedia.org/wiki/Oriental_Shorthair"),
    ("Persian", "https://en.wikipedia.org/wiki/Persian_(cat)"),
    ("Pixie-bob", "https://en.wikipedia.org/wiki/Pixiebob"),
    ("Ragamuffin", "https://en.wikipedia.org/wiki/Ragamuffin_cat"),
    ("Ragdoll", "https://en.wikipedia.org/wiki/Ragdoll"),
    ("Russian Blue", "https://en.wikipedia.org/wiki/Russian_Blue"),
    ("Savannah", "https://en.wikipedia.org/wiki/Savannah_cat"),
    ("Scottish Fold", "https://en.wikipedia.org/wiki/Scottish_Fold"),
    ("Selkirk Rex", "https://en.wikipedia.org/wiki/Selkirk_Rex"),
    ("Siamese", "https://en.wikipedia.org/wiki/Siamese_(cat)"),
    ("Siberian", "https://en.wikipedia.org/wiki/Siberian_(cat)"),
    ("Singapura", "https://en.wikipedia.org/wiki/Singapura_(cat)"),
    ("Snowshoe", "https://en.wikipedia.org/wiki/Snowshoe_(cat)"),
    ("Somali", "https://en.wikipedia.org/wiki/Somali_(cat)"),
    ("Sphynx", "https://en.wikipedia.org/wiki/Sphynx_(cat)"),
    ("Tokinese", "https://en.wikipedia.org/wiki/Tonkinese_(cat)"),
    ("Toyger", "https://en.wikipedia.org/wiki/Toyger"),
    ("Turkish Angora", "https://en.wikipedia.org/wiki/Turkish_Angora"),
    ("Turkish Van", "https://en.wikipedia.org/wiki/Turkish_Van"),
    ("York Chocolate", "https://en.wikipedia.org/wiki/York_Chocolate"),
];

const BREED_LIST: [&str; 67] = [
    "abys", "aege", "abob", "acur", "asho", "awir", "amau", "amis", "bali", "bamb", "beng", "birm",
    "bomb", "bslo", "bsho", "bure", "buri", "cspa", "ctif", "char", "chau", "chee", "csho", "crex",
    "cymr", "cypr", "drex", "dons", "lihu", "emau", "ebur", "esho", "hbro", "hima", "jbob", "java",
    "khao", "kora", "kuri", "lape", "mcoo", "mala", "manx", "munc", "nebe", "norw", "ocic", "orie",
    "pers", "pixi", "raga", "ragd", "rblu", "sava", "sfol", "srex", "siam", "sibe", "sing", "snow",
    "soma", "sphy", "tonk", "toyg", "tang", "tvan", "ycho",
];

const HELP_MSG: &str = "Small app to fetch cat pics from TheCatAPI

\x1b[92m\x1b[1mUSAGE:\x1b[0m
    \x1b[96m\x1b[1mvalerian\x1b[0m \x1b[96m[FLAGS] [OPTIONS]\x1b[0m

\x1b[92m\x1b[1mFLAGS:\x1b[0m
   \x1b[1m\x1b[96m-h  | --help\x1b[0m               prints help information
   \x1b[1m\x1b[96m-bl | --breed-list\x1b[0m         prints breeds ids

\x1b[92m\x1b[1mOPTIONS:\x1b[0m

    \x1b[96m\x1b[1m-b | --breed\x1b[0m        specify breed to fetch [default: None]
    \x1b[96m\x1b[1m-c | --count\x1b[0m        number of cats to fetch and download [Required]
    \x1b[96m\x1b[1m-o | --output\x1b[0m       output directory [default: current working directory]";
