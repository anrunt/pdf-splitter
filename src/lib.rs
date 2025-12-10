use std::{error::Error, io::{self, Write}};
use pdfcat::{config::{CompressionLevel, Config, Metadata, OverwriteMode, PageRange}, validation::Validator};
use std::path::PathBuf;

pub struct UserConfig {
    pub path: String,
    pub start_range: u32,
    pub end_range: u32
}

impl UserConfig {
    pub fn build() -> Result<UserConfig, Box<dyn Error>> {
        let mut input: String = String::new();

        print!("Give start and end range separated by space: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        if input.trim().is_empty() {
            return Err("No ranges provided - expected 2".into());
        }

        let mut iter = input.split_whitespace();

        // Check if start_range is not bigger than end range
        let start_range: u32 = iter.next()
            .ok_or("Missing range argument (should be 2 only 1 provided)")?
            .parse()
            .map_err(|_| "Erorr with start_range")?;

        let end_range: u32 = iter.next()
            .ok_or("Missing range argument (should be 2 only 1 provided)")?
            .parse()
            .map_err(|_| "Error with end_range")?;

        if iter.next().is_some() {
            return Err("Too many arguments - expected exactly 2".into())
        };

        if start_range == 0 || end_range == 0 {
            return Err("Range has to be greater or equal to 1".into());
        }

        if start_range > end_range {
            return Err("Start range can't be bigger than end range".into());
        }

        input.clear();

        print!("Give pdf filename: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        let path = parse_and_validate_filename(&input)?;

//        println!("Start_range: {}, end_range: {}, path: {}", start_range, end_range, path);

        Ok(UserConfig {start_range, end_range, path})
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

pub async fn extract_pages(config: &UserConfig) -> Result<(), Box<dyn Error>> {
    let page_range_string = format!("{}-{}", &config.start_range, &config.end_range); 
    let page_range = PageRange::parse(&page_range_string)
        .map_err(|e| format!("Failed to parse page range {}:{}", page_range_string, e))?;

    let validator = Validator::new();
    let validation_result = validator.validate_file(&PathBuf::from(&config.path)).await?;

    if config.end_range > validation_result.page_count as u32 {
        return Err("End range exceeds number of pages".into());
    }

    if config.start_range > validation_result.page_count as u32 {
        return Err("Start range exceeds number of pages".into());
    }

    let path_buf = PathBuf::from(&config.path);
    let path_buf_name = path_buf.file_stem()
        .ok_or("Can't get file name")?
        .to_string_lossy();
    let path_buf_extension = path_buf.extension()
        .ok_or("Can't get file extension")?
        .to_string_lossy();

    let output_name = format!("{}-{}-{}.{}", path_buf_name, &config.start_range, &config.end_range, path_buf_extension);

    let pdfcat_config = Config {
        inputs: vec![PathBuf::from(&config.path)],
        output: PathBuf::from(output_name),
        page_range: Some(page_range),
        dry_run: false,
        verbose: false,
        overwrite_mode: OverwriteMode::Prompt,
        quiet: false,
        bookmarks: true,
        compression: CompressionLevel::Standard,
        metadata: Metadata::default(),
        continue_on_error: false,
        jobs: None,
        rotation: None,
    };

    let (mut document, stats) = pdfcat::merge::merge_pdfs(&pdfcat_config).await?;
    document.save(&pdfcat_config.output)?;
    println!("Extracted {} pages", stats.total_pages);
    Ok(())
}