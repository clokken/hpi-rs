use hpi::HpiError;

#[derive(Debug)]
pub enum CliError {
    BadCommand(String),
    HpiError(HpiError),
    Unknown(),
}
