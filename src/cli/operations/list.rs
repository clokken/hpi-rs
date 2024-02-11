use hpi::{HpiReader, HpiItem};
use regex::Regex;

use crate::cli::{file_utils, pattern_matcher, CliError};

#[derive(Debug)]
pub struct ListOptions {
    pub input_hpi_path: String,
    pub only_files: bool,
    pub only_dirs: bool,
    pub match_pattern: Option<String>,
    pub match_regex: Option<Regex>,
}

pub fn list(options: &ListOptions) -> Result<(), CliError> {
    let data = file_utils::read_file_into_bytes(&options.input_hpi_path).map_err(|err| {
        let msg = format!(
            "Failed to read file at path \"{}\". Error: {}",
            options.input_hpi_path,
            err.to_string()
        );

        CliError::IoError(msg)
    })?;

    let ctx = HpiReader::read(&data).map_err(CliError::HpiError)?;

    ctx.root.iter(false).for_each(|item| next_item(options, item));

    Ok(())
}

fn next_item(options: &ListOptions, item: &HpiItem) {
    print_item(options, item);

    if let HpiItem::Directory(dir) = item {
        dir.iter(false).for_each(|sub_item| next_item(options, sub_item));
    }
}

fn print_item(options: &ListOptions, item: &HpiItem) {
    let path_name = match item {
        HpiItem::Directory(dir) => {
            if options.only_files {
                return;
            }

            format!("{}/{}/", dir.path, dir.name)
        },
        HpiItem::Entry(entry) => {
            if options.only_dirs {
                return;
            }

            format!("{}/{}", entry.path, entry.name)
        }
    };

    if let Some(pattern) = &options.match_pattern {
        if !pattern_matcher::test(pattern, &path_name) {
            return;
        }
    }

    if let Some(regex) = &options.match_regex {
        if !regex.is_match(&path_name) {
            return;
        }
    }

    println!("{}", path_name);
}
