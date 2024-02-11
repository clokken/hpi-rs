use hpi::HpiError;

#[derive(Debug)]
pub enum CliError {
    IoError(String),
    BadCommand(String),
    HpiError(HpiError),
    Unknown(),
}
