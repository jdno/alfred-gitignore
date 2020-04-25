use crate::repository::{Repository, Template};
use std::collections::HashMap;
use std::io::Error;

/// Represents a user's input.
///
/// The query represents a user's input, and provides abstractions to interact with the input in a
/// meaningful way. First, it provides a way to sanitize the input and return only the elements that
/// match templates in the repository. Second, it can make suggestions for the last phrase the user
/// is entering.
#[derive(Debug)]
pub struct Query {
    query: Vec<String>,
    query_map: HashMap<String, String>,
    templates_map: HashMap<String, Template>,
}

impl Query {
    /// Returns a new query object.
    ///
    /// The query represents the users input. It is analyzed after each keystroke to provide
    /// suggestions, and filter the list of templates.
    pub fn new(repository: &Repository, query: Option<Vec<&str>>) -> Result<Self, Error> {
        let query = match query {
            Some(values) => values.iter().map(|str| String::from(*str)).collect(),
            None => Vec::new(),
        };

        let mut query_map = HashMap::new();
        for string in &query {
            query_map.insert(string.to_lowercase(), string.clone());
        }

        let mut templates_map = HashMap::new();
        let templates = repository.templates()?;
        for template in templates {
            templates_map.insert(template.name().to_lowercase(), template);
        }

        Ok(Query {
            query,
            query_map,
            templates_map,
        })
    }

    /// Returns a list of valid template names.
    ///
    /// Since users can provide arbitrary strings as a query, this method exists to filter their
    /// input to a list of templates that exist in the repository.
    pub fn sanitized_query(&self) -> Vec<String> {
        self.query_map
            .iter()
            .filter(|(key, _query)| self.templates_map.contains_key(*key))
            .map(|(_key, query)| query)
            .cloned()
            .collect()
    }

    /// Returns suggestions for the last element in the query.
    ///
    /// Alfred can be configured to execute `alfred-gitignore` for every key that a user types. This
    /// makes it possible to offer suggestions and autocomplete for their input. This method takes
    /// the last element in the query, and matches it against the templates.
    ///
    /// If only a single match is found, it indicates that the last element is already correct and
    /// done. In this case, all templates except the ones in the current query are returned.
    pub fn suggestions(&self) -> Vec<String> {
        let last_element = match self.query.last() {
            Some(element) => element.to_lowercase(),
            None => {
                return self
                    .templates_map
                    .iter()
                    .map(|(_key, template)| template.name())
                    .cloned()
                    .collect()
            }
        };

        if self.templates_map.contains_key(last_element.as_str()) {
            self.templates_map
                .iter()
                .filter(|(key, _template)| !self.query_map.contains_key(*key))
                .map(|(_key, template)| template.name())
                .cloned()
                .collect()
        } else {
            self.templates_map
                .iter()
                .filter(|(_key, template)| template.comparator().starts_with(last_element.as_str()))
                .map(|(_key, template)| template.name())
                .cloned()
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::query::Query;
    use crate::testing::initialize_repository;
    use tempdir::TempDir;

    #[test]
    fn new_without_query() {
        let tempdir = TempDir::new("alfred-gitignore").unwrap();
        let repository = initialize_repository(tempdir.path()).unwrap();

        let query = Query::new(&repository, None).unwrap();

        assert_eq!(0, query.query.len());
        assert_eq!(0, query.query_map.len());
        assert_eq!(2, query.templates_map.len());
    }

    #[test]
    fn new_with_query() {
        let tempdir = TempDir::new("alfred-gitignore").unwrap();
        let repository = initialize_repository(tempdir.path()).unwrap();

        let query = Query::new(&repository, Some(vec!["apples"])).unwrap();

        assert_eq!(1, query.query.len());
        assert_eq!(1, query.query_map.len());
        assert_eq!(2, query.templates_map.len());

        assert_eq!(&String::from("apples"), query.query.first().unwrap());
    }

    #[test]
    fn sanitized_query() {
        let tempdir = TempDir::new("alfred-gitignore").unwrap();
        let repository = initialize_repository(tempdir.path()).unwrap();

        let query = Query::new(&repository, Some(vec!["Apples", "Peaches"]))
            .unwrap()
            .sanitized_query();

        assert_eq!(1, query.len());
        assert_eq!(&String::from("Apples"), query.first().unwrap())
    }

    #[test]
    fn suggestions_without_query() {
        let tempdir = TempDir::new("alfred-gitignore").unwrap();
        let repository = initialize_repository(tempdir.path()).unwrap();

        let query = Query::new(&repository, Some(Vec::new())).unwrap();
        let suggestions = query.suggestions();

        assert_eq!(2, suggestions.len());
        assert_eq!(vec!["apples", "oranges"], suggestions);
    }

    #[test]
    fn suggestions_without_match() {
        let tempdir = TempDir::new("alfred-gitignore").unwrap();
        let repository = initialize_repository(tempdir.path()).unwrap();

        let query = Query::new(&repository, Some(vec!["Apples"])).unwrap();
        let suggestions = query.suggestions();

        assert_eq!(1, suggestions.len());
        assert_eq!(vec!["oranges"], suggestions);
    }

    #[test]
    fn suggestions_with_match() {
        let tempdir = TempDir::new("alfred-gitignore").unwrap();
        let repository = initialize_repository(tempdir.path()).unwrap();

        let query = Query::new(&repository, Some(vec!["Or"])).unwrap();
        let suggestions = query.suggestions();

        assert_eq!(1, suggestions.len());
        assert_eq!(vec!["oranges"], suggestions);
    }
}
