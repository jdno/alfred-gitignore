use crate::builder::Builder;
use crate::exit_with_error;
use crate::query::Query;
use crate::repository::Repository;
use alfred::{Item, ItemBuilder, ItemType};
use clap::Values;
use std::fs::read_to_string;
use std::io::stdout;
use std::process::exit;

pub struct Build<'a> {
    _lifetime: &'a str,
}

impl<'a> Build<'a> {
    pub fn item(query: &str) -> Item<'a> {
        ItemBuilder::new("Create .gitignore file")
            .subtitle("Combine the selected templates into a single .gitignore file")
            .autocomplete(format!("--build {query}"))
            .valid(false)
            .into_item()
    }

    pub fn perform(repository: Repository, selections: Option<Values>) -> ! {
        let selections = selections.map(|values| values.collect());
        let query = match Query::new(&repository, selections) {
            Ok(query) => query,
            Err(error) => exit_with_error(&error),
        };

        let builder = Builder::new(repository, &query);
        let path = match builder.build() {
            Ok(path) => path,
            Err(error) => exit_with_error(&error),
        };
        let path_str = path.to_str().unwrap();

        alfred::json::write_items(
            stdout(),
            &[
                ItemBuilder::new("Open .gitignore file")
                    .arg(path_str)
                    .type_(ItemType::File)
                    .icon_file(path_str)
                    .into_item(),
                ItemBuilder::new("Copy to clipboard")
                    .arg(read_to_string(&path).unwrap())
                    .into_item(),
            ],
        )
        .unwrap();

        exit(0);
    }
}
