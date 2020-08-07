use super::build::Build;
use super::update::Update;
use crate::exit_with_error;
use crate::query::Query;
use crate::repository::Repository;
use alfred::ItemBuilder;
use clap::Values;
use std::io::stdout;
use std::process::exit;

pub struct Select<'a> {
    _lifetime: &'a str,
}

impl<'a> Select<'a> {
    pub fn perform(repository: &Repository, selections: Option<Values>) -> ! {
        let selections = match selections {
            Some(values) => Some(values.collect()),
            None => None,
        };
        let query = match Query::new(repository, selections) {
            Ok(query) => query,
            Err(error) => exit_with_error(&error),
        };
        let query_string = Select::construct_query_string(&query);

        let mut items = if query.sanitized_query().is_empty() {
            vec![Update::item()]
        } else {
            vec![Build::item(&query_string)]
        };

        for suggestion in query.suggestions() {
            let mut autocomplete = query_string.clone();
            autocomplete.push_str(" ");
            autocomplete.push_str(&suggestion);

            items.push(
                ItemBuilder::new(suggestion)
                    .autocomplete(autocomplete)
                    .valid(false)
                    .into_item(),
            );
        }

        alfred::json::write_items(stdout(), &items).unwrap();

        exit(0);
    }

    fn construct_query_string(query: &Query) -> String {
        let mut query_string = String::new();
        let sanitized_query = query.sanitized_query();

        if !sanitized_query.is_empty() {
            let names: Vec<String> = query
                .sanitized_query()
                .iter()
                .map(|template| template.name())
                .cloned()
                .collect();

            query_string.push_str(" ");
            query_string.push_str(&names.join(" "));
        }

        query_string
    }
}
