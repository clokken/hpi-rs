use super::CliError;
use super::file_utils;

#[derive(Debug)]
pub struct ListOptions {
    pub input_hpi_path: String,
    pub only_files: bool,
    pub only_dirs: bool,
    pub match_pattern: Option<String>,
    pub match_regexp: Option<String>,
}

pub fn list(options: &ListOptions) -> Result<(), CliError> {
    println!("Listing...");
    dbg!(options);
    Ok(())
}
