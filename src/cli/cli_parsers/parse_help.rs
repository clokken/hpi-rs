use crate::cli::CliOperation;

pub fn parse_help(sub_args: &[String]) -> Result<CliOperation, String> {
    Ok(CliOperation::Help)
}
