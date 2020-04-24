use crate::repository::Repository;
use alfred::ItemBuilder;
use clap::{crate_version, App, Arg, SubCommand};
use std::io::{stdout, Error, ErrorKind};
use std::path::PathBuf;
use std::process::exit;

mod repository;

const UPDATE_COMMAND: &str = "update";

fn main() {
    let matches = App::new("alfred-gitignore")
        .version(crate_version!())
        .author("Jan David <jandavid@6a64.com>")
        .about("An Alfred workflow to generate .gitignore files")
        .arg(
            Arg::with_name("repository")
                .help("Provide a custom directory for development and testing")
                .short("r")
                .long("repository")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name(UPDATE_COMMAND)
                .about("Download the latest .gitignore templates from GitHub"),
        )
        .get_matches();

    let _repository = initialize_repository(matches.value_of("repository"));
}

fn initialize_repository(path: Option<&str>) -> Repository {
    let repository_path = match path {
        Some(path) => PathBuf::from(path),
        None => match alfred::env::workflow_data() {
            Some(path) => path,
            None => exit_with_error(&Error::new(
                ErrorKind::NotFound,
                "Alfred did not provide a data directory to the workflow",
            )),
        },
    };

    match Repository::new(repository_path) {
        Ok(repository) => repository,
        Err(error) => exit_with_error(&error),
    }
}

fn exit_with_error(error: &Error) -> ! {
    alfred::json::write_items(
        stdout(),
        &[ItemBuilder::new("Error running gitignore workflow")
            .subtitle(error.to_string())
            .into_item()],
    )
    .unwrap();

    exit(1);
}
