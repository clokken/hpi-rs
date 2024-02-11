use super::CliError;

pub struct ExtractOptions {
    pub input_hpi_path: String,
    pub files: Vec<String>,
    pub out_path: String,
    pub force: bool,
}

pub fn extract(options: &ExtractOptions) -> Result<(), CliError> {
    println!("Extracting...");
    Ok(())
}
