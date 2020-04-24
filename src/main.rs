use crate::repository::Repository;
use clap::{crate_version, App, Arg, SubCommand};
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
            None => {
                eprintln!("Failed to set up data directory. Either run the CLI through Alfred, or pass the --repository option.");
                exit(1);
            }
        },
    };

    match Repository::new(repository_path) {
        Ok(repository) => repository,
        Err(error) => {
            eprintln!("Failed to initialize repository with the following error:");
            eprintln!("{}", error);
            exit(1);
        }
    }
}
