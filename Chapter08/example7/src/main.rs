extern crate failure;

use std::fs::File;
use std::io::Read;

use failure::{Error, ResultExt};

fn main() {
    match read_file() {
        Err(e) => {
            eprintln!("Error: {}", e);

            for cause in e.causes().skip(1) {
                eprintln!("Caused by: {}", cause);
            }
        }
        Ok(content) => {
            println!("{}…", content.chars().take(15).collect::<String>());
        }
    }
}

fn read_file() -> Result<String, Error> {
    let file_name = "Cargo.toml";
    let mut file = File::open(file_name).context("error opening the file")?;

    let mut content = String::new();
    file.read_to_string(&mut content)
        .context("error reading the file")?;

    Ok(content)
}
