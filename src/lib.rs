use std::{error::Error, io::{self, Write}};
pub struct Config {
    path: String,
    start_range: i32,
    end_range: i32
}

impl Config {
    pub fn build() -> Result<Config, Box<dyn Error>> {
        let mut input: String = String::new();

        print!("Give start and end range separated by space: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        if input.trim().is_empty() {
            return Err("No ranges provided - expected 2".into());
        }

        let mut iter = input.split_whitespace();

        // Check if start_range is not bigger than end range
        let start_range: i32 = iter.next()
            .ok_or("Missing range argument (should be 2 only 1 provided)")?
            .parse()
            .map_err(|_| "Erorr with start_range")?;

        let end_range: i32 = iter.next()
            .ok_or("Missing range argument (should be 2 only 1 provided)")?
            .parse()
            .map_err(|_| "Error with end_range")?;

        if iter.next().is_some() {
            return Err("Too many arguments - expected exactly 2".into())
        };

        if start_range > end_range {
            return Err("Start range can't be bigger than end range".into());
        }

        input.clear();

        print!("Give pdf filename: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        let path = parse_and_validate_filename(&input)?;

        println!("Start_range: {}, end_range: {}, path: {}", start_range, end_range, path);

        Ok(Config {start_range, end_range, path})
    }
}

pub fn parse_and_validate_filename(input: &str) -> Result<String, Box<dyn Error>> {
    let path_trimmed_string= input.trim().to_string();
    let path = Some(path_trimmed_string).filter(|p| !p.is_empty()).ok_or("Empty pdf name")?;

    if path.starts_with(".") {
        return Err("Filename cant start with .".into());
    }

    if path.chars().any(|c| c == '/' || c == '\\' || c == '~' || c == ' ') {
        return Err("Filename cant contain /\\~ and whitespaces".into());
    }

    let dot_count = path.chars().filter(|&c| c == '.').count();

    if dot_count > 1 {
        return Err("Filename contains more than one dot!".into());
    }

    if !path.ends_with(".pdf") {
        return Err("File must be of pdf type".into());
    }

    Ok(path)
}