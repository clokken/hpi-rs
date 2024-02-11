use crate::cli::CliOperation;

pub fn parse_help(_sub_args: &[String]) -> Result<CliOperation, String> {
    Ok(CliOperation::Help)
}
