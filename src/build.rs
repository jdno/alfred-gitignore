use crate::builder::Builder;
use crate::exit_with_error;
use crate::query::Query;
use crate::repository::Repository;
use alfred::{Item, ItemBuilder};
use clap::Values;
use std::fs::read_to_string;
use std::process::exit;

pub struct Build<'a> {
    _lifetime: &'a str,
}

impl<'a> Build<'a> {
    pub fn item(query: &str) -> Item<'a> {
        ItemBuilder::new("Create .gitignore file")
            .subtitle("Combine the selected templates into a single .gitignore file")
            .arg(format!("--build {}", query))
            .into_item()
    }

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

        println!("{}", read_to_string(&path).unwrap());

        exit(0);
    }
}
