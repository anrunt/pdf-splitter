use std::error::Error;

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

    Ok(path)
}