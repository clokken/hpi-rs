mod cli;

use std::process;
use cli::{CliOperation, CliError};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let operation = cli::parse(&args).unwrap_or_else(|err| {
        eprintln!("Error parsing your command:");
        eprintln!("{}", err);
        process::exit(1);
    });

    let result = match operation {
        CliOperation::List(options) => {
            cli::list(&options)
        },
        CliOperation::Extract(options) => {
            cli::extract(&options)
        },
        CliOperation::Help => {
            cli::help();
            process::exit(0);
        },
    };

    if let Err(err) = result {
        match err {
            CliError::BadCommand(msg) => {
                eprintln!("An error occurred executing your command:");
                eprintln!("{}", msg);
            },
            CliError::IoError(msg) => {
                eprintln!("An IO error:");
                eprintln!("{}", msg);
            },
            CliError::HpiError(err) => {
                eprintln!("An HPI error has occurred:");
                // TODO handle the error properly
                eprintln!("{}", err);
            },
            CliError::Unknown() => {
                eprintln!("An unknown error has occurred :(");
            },
        };

        process::exit(1);
    }
}
