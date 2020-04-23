use getset::Getters;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;

/// The repository with `.gitignore` files
///
/// The repository is a copy of the [github/gitignore](https://github.com/github/gitignore) Git
/// repository, and contains all `.gitignore` files. A local copy of the remote repository is
/// maintained on disk to speed up searches and to provide offline capabilities.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, Getters)]
pub struct Repository {
    /// Returns the path to the repository.
    #[getset(get = "pub")]
    path: PathBuf,
}

impl Repository {
    /// Returns a new instance for the repository at the given location.
    ///
    /// When initializing a repository, a number of prerequisites is checked to ensure the path
    /// points to a valid repository. A result is returned that either contains an instance of the
    /// repository, or an error that explains why the repository could not be initialized.
    pub fn new(path: PathBuf) -> Result<Self, Error> {
        if !path.exists() {
            return Err(Error::new(
                ErrorKind::NotFound,
                "Path to repository does not exist",
            ));
        }

        Ok(Repository { path })
    }
}

#[cfg(test)]
mod tests {
    use crate::repository::Repository;
    use std::io::ErrorKind;
    use std::path::PathBuf;
    use tempdir::TempDir;

    #[test]
    fn new_with_existing_path() {
        let directory = TempDir::new("new_with_existing_path").unwrap();
        let repository = Repository::new(directory.into_path());

        assert!(repository.is_ok())
    }

    #[test]
    fn new_with_empty_path() {
        let repository = Repository::new(PathBuf::from("does-not-exist"));

        assert_eq!(ErrorKind::NotFound, repository.unwrap_err().kind());
    }
}
