use super::CliOperation;

use super::cli_parsers;

struct Operation {
    aliases: [&'static str; 2],
    func: fn (&[String]) -> Result<CliOperation, String>,
}

const OPERATIONS: &'static [Operation] = &[
    Operation { aliases: ["l", "list"],       func: cli_parsers::parse_list },
    Operation { aliases: ["e", "extract"],    func: cli_parsers::parse_extract },
    Operation { aliases: ["h", "help"],       func: cli_parsers::parse_help },
];

pub fn parse(args: &[String]) -> Result<CliOperation, String> {
    if args.len() < 2 {
        return Err("Not enough arguments.".to_string());
    }

    let operation_name = &args[1].to_lowercase();

    let operation_func = OPERATIONS.iter()
        .find(|op| op.aliases.contains(&operation_name.as_str()))
        .map(|op| op.func);

    match operation_func {
        Some(func) => {
            let sub_args = &args[2..];
            func(sub_args)
        }
        None => Err(format!("Unknown operation: \"{}\"", operation_name)),
    }
}
