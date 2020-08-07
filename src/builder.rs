use crate::query::Query;
use crate::repository::{Repository, Template};
use std::collections::hash_map::DefaultHasher;
use std::env::temp_dir;
use std::fs::{read_to_string, File};
use std::hash::{Hash, Hasher};
use std::io::{Error, Write};
use std::path::PathBuf;

const FILE_NAME_PREFIX: &str = "alfred-gitignore-";

/// Constructs a `.gitignore` file from a query.
///
/// The builder combines the `*.gitignore` templates in a query into a single `.gitignore` file.
pub struct Builder {
    repository: Repository,
    templates: Vec<Template>,
}

impl Builder {
    /// Returns a new builder that can turn the query into a single `.gitignore` file.
    pub fn new(repository: Repository, query: &Query) -> Self {
        Builder {
            repository,
            templates: query.sanitized_query(),
        }
    }

    /// Build a single `.gitignore` file from the query.
    ///
    /// The templates selected in the query are written to a single `.gitignore` file at a temporary
    /// location on the local disk, and the path to the file is returned.
    pub fn build(&self) -> Result<PathBuf, Error> {
        let destination = temp_dir().join(self.file_name());
        let mut file = File::create(&destination)?;

        for i in 0..self.templates.len() {
            if let Some(template) = self.templates.get(i) {
                let template_path = self.repository.path().join(template.file_name());
                let content = read_to_string(template_path)?;

                file.write_all(format!("### {}\n", template.file_name()).as_bytes())?;
                file.write_all(&content.as_bytes())?;

                if i < self.templates.len() - 1 {
                    file.write_all(b"\n")?;
                }
            }
        }

        file.sync_all()?;

        Ok(destination)
    }

    /// Returns the file name for the given query.
    ///
    /// The names of the templates in the given query are concatenated and then hashed to allow the
    /// results of the query to be cached.
    fn file_name(&self) -> PathBuf {
        let mut template_names: Vec<String> = self
            .templates
            .iter()
            .map(|template| template.name())
            .cloned()
            .collect();

        template_names.sort();

        let mut hasher = DefaultHasher::new();
        template_names.join("").hash(&mut hasher);
        let hash = hasher.finish();

        let mut file_name = String::from(FILE_NAME_PREFIX);
        file_name.push_str(&hash.to_string());
        file_name.push_str(".gitignore");

        PathBuf::from(file_name)
    }
}

#[cfg(test)]
mod tests {
    use crate::builder::Builder;
    use crate::query::Query;
    use crate::testing::initialize_repository;
    use std::fs::read_to_string;
    use tempfile::TempDir;

    #[test]
    fn build_one_file() {
        let tempdir = TempDir::new().unwrap();
        let repository = initialize_repository(tempdir.path()).unwrap();
        let query = Query::new(&repository, Some(vec!["apples"])).unwrap();

        let builder = Builder::new(repository, &query);
        let path = builder.build().unwrap();

        let mut expected = String::new();
        expected.push_str("### apples.gitignore\n");
        expected.push_str(include_str!("../tests/files/repository/apples.gitignore"));

        let content = read_to_string(path).unwrap();

        assert_eq!(expected, content);
    }

    #[test]
    fn build_two_files() {
        let tempdir = TempDir::new().unwrap();
        let repository = initialize_repository(tempdir.path()).unwrap();
        let query = Query::new(&repository, Some(vec!["oranges", "apples"])).unwrap();

        let builder = Builder::new(repository, &query);
        let path = builder.build().unwrap();

        let mut expected = String::new();
        expected.push_str("### oranges.gitignore\n");
        expected.push_str(include_str!("../tests/files/repository/oranges.gitignore"));

        expected.push_str("\n### apples.gitignore\n");
        expected.push_str(include_str!("../tests/files/repository/apples.gitignore"));

        let content = read_to_string(path).unwrap();
        assert_eq!(expected, content);
    }

    #[test]
    fn file_name() {
        let tempdir = TempDir::new().unwrap();
        let repository = initialize_repository(tempdir.path()).unwrap();
        let query = Query::new(&repository, Some(vec!["apples"])).unwrap();

        let builder = Builder::new(repository, &query);

        assert_eq!(
            "alfred-gitignore-16623996710012718148.gitignore",
            builder.file_name().to_str().unwrap()
        );
    }
}
