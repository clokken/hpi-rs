use crate::cli::{CliOperation, ListOptions};
use regex::Regex;

pub fn parse_list(sub_args: &[String]) -> Result<CliOperation, String> {
    if sub_args.len() < 1 {
        return Err("Missing input HPI file argument.".to_string());
    }

    let input_hpi_path = (&sub_args[0]).to_string();
    let mut only_dirs = false;
    let mut only_files = false;
    let mut match_pattern: Option<String> = None;
    let mut match_regex: Option<Regex> = None;
    let mut skip_next_arg = false;

    let op_args = &sub_args[1..];

    for (index, arg) in op_args.iter().enumerate() {
        if skip_next_arg {
            skip_next_arg = false;
            continue;
        }

        let is_last_arg = index == op_args.len() - 1;

        if arg == "--only-files" || arg == "-of" {
            only_files = true;
            continue;
        }

        if arg == "--only-dirs" || arg == "-od" {
            only_dirs = true;
            continue;
        }

        if arg == "--match" || arg == "-m" {
            if is_last_arg {
                return Err("Missing pattern after --match parameter".to_string());
            }

            let next_arg = &op_args[index + 1];
            match_pattern = Some(next_arg.to_string());

            skip_next_arg = true;
            continue;
        }

        if arg == "--regex" || arg == "-r" {
            if is_last_arg {
                return Err("Missing regexp after --regex parameter".to_string());
            }

            let next_arg = &op_args[index + 1];

            match_regex = match Regex::new(next_arg) {
                Ok(regex) => Some(regex),
                Err(err) => {
                    return Err(format!("Error parsing regex: {}\nRegex error: {err}", next_arg));
                },
            };

            skip_next_arg = true;
            continue;
        }

        return Err(format!("Unknown argument: {}", arg));
    }

    if only_files && only_dirs {
        return Err("You cannot use --only-files and --only-dirs simultaneously.".to_string());
    }

    let options = ListOptions {
        input_hpi_path,
        only_dirs,
        only_files,
        match_pattern,
        match_regex,
    };

    Ok(CliOperation::List(options))
}

// TODO some basic tests
