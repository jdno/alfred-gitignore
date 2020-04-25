use crate::builder::Builder;
use crate::exit_with_error;
use crate::query::Query;
use crate::repository::Repository;
use alfred::{ItemBuilder, ItemType};
use clap::Values;
use std::fs::read_to_string;
use std::io::stdout;
use std::process::exit;

pub struct Build<'a> {
    _lifetime: &'a str,
}

impl<'a> Build<'a> {
    pub fn perform(repository: Repository, selections: Option<Values>) -> ! {
        let selections = match selections {
            Some(values) => Some(values.collect()),
            None => None,
        };
        let query = match Query::new(&repository, selections) {
            Ok(query) => query,
            Err(error) => exit_with_error(&error),
        };

        let builder = Builder::new(repository, &query);
        let path = match builder.build() {
            Ok(path) => path,
            Err(error) => exit_with_error(&error),
        };

        alfred::json::write_items(
            stdout(),
            &[
                ItemBuilder::new("Open .gitignore file")
                    .arg(path.clone().to_str().unwrap())
                    .type_(ItemType::File)
                    .into_item(),
                ItemBuilder::new("Copy to clipboard")
                    .subtitle("Press ⌘C to copy the .gitignore file")
                    .text_copy(read_to_string(&path).unwrap())
                    .valid(false)
                    .into_item(),
            ],
        )
        .unwrap();

        exit(0);
    }
}
