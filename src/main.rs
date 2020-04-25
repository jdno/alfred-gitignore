use crate::build::Build;
use crate::repository::Repository;
use crate::select::Select;
use crate::update::Update;
use alfred::ItemBuilder;
use clap::{crate_version, App, Arg};
use std::io::{stdout, Error, ErrorKind};
use std::path::PathBuf;
use std::process::exit;

mod build;
mod select;
mod update;

mod builder;
mod query;
mod repository;

#[cfg(test)]
mod testing;

const TEMPLATES_ARG: &str = "TEMPLATES";

const BUILD_COMMAND: &str = "build";
const SELECT_COMMAND: &str = "select";
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
        .arg(
            Arg::with_name(BUILD_COMMAND)
                .help("Create a single .gitignore file from the templates")
                .short("b")
                .long("build"),
        )
        .arg(
            Arg::with_name(SELECT_COMMAND)
                .help("Select templates to combine them into a single file")
                .short("s")
                .long("select"),
        )
        .arg(
            Arg::with_name(UPDATE_COMMAND)
                .help("Update the repository or workflow data directory")
                .short("u")
                .long(UPDATE_COMMAND),
        )
        .arg(
            Arg::with_name(TEMPLATES_ARG)
                .help("Provide a list of templates")
                .multiple(true),
        )
        .get_matches();

    let repository = initialize_repository(matches.value_of("repository"));

    if matches.is_present("build") {
        Build::perform(repository, matches.values_of(TEMPLATES_ARG));
    }

    if matches.is_present("update") {
        Update::perform(&repository);
    }

    if matches.is_present("select") {
        Select::perform(&repository, matches.values_of(TEMPLATES_ARG));
    }

    alfred::json::write_items(stdout(), &[Select::item(), Update::item()]).unwrap();
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
