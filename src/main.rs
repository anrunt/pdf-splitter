use std::{error::Error, io::{self, Write}};

use pdf_splitter::parse_and_validate_filename;

fn main() {
    let config = Config::build();
    match config {
        Ok(_) => println!("Successfully built config"),
        Err(e) => {
            println!("Error with building config: {e}");
            std::process::exit(1);
        }
    }
}

struct Config {
    path: String,
    start_range: i32,
    end_range: i32
}

impl Config {
    fn build() -> Result<Config, Box<dyn Error>> {
        let mut input: String = String::new();

        print!("Give start and end range separated by space: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        let mut iter = input.split_whitespace();

        let start_range: i32 = iter.next()
            .ok_or("Missing range argument (should be 2 only 1 provided)")?
            .parse()
            .map_err(|_| "Erorr with start_range")?;

        let end_range: i32 = iter.next()
            .ok_or("Missing range argument (should be 2 only 1 provided)")?
            .parse()
            .map_err(|_| "Error with end_range")?;

        input.clear();

        print!("Give pdf filename: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        let path = parse_and_validate_filename(&input)?;

        println!("Start_range: {}, end_range: {}, path: {}", start_range, end_range, path);

        Ok(Config {start_range, end_range, path})
    }
}