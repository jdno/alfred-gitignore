use getset::Getters;
use std::env::temp_dir;
use std::fs::{create_dir, read_dir, File};
use std::io::{copy, Error, ErrorKind};
use std::path::PathBuf;
use zip::ZipArchive;

const ARCHIVE: &str = "https://github.com/github/gitignore/archive/master.zip";

/// A template represents a `*.gitignore` file in a repository.
#[derive(Clone, Debug, Getters)]
pub struct Template {
    /// Returns the name of the template.
    #[getset(get = "pub")]
    name: String,

    /// Returns the file name of the template.
    #[getset(get = "pub")]
    file_name: String,

    /// Returns a sanitized version of the template's name for comparisons.
    #[getset(get = "pub")]
    comparator: String,
}

impl Template {
    /// Returns a new instance of a template for the given file name.
    pub fn new(file_name: &str) -> Template {
        let name = file_name.replace(".gitignore", "");

        Template {
            comparator: name.to_lowercase(),
            file_name: String::from(file_name),
            name,
        }
    }
}

/// The repository with `.gitignore` files
///
/// The repository is a copy of the [github/gitignore](https://github.com/github/gitignore) Git
/// repository, and contains all `.gitignore` files. A local copy of the remote repository is
/// maintained on disk to speed up searches and to provide offline capabilities.
#[derive(Debug, Getters)]
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
            create_dir(&path)?;
        }

        Ok(Repository { path })
    }

    /// Returns a list of templates in the repository.
    ///
    /// The templates in a repository are all the `*.gitignore` files in its path. Since this is a
    /// convention, only the base names of the files are returned without their ending. For example,
    /// only `GitHub` is returned for the template `GitHub.gitignore`.
    pub fn templates(&self) -> Result<Vec<Template>, Error> {
        let entries = read_dir(self.path())?;

        let mut templates: Vec<Template> = entries
            .filter_map(|entry| {
                let entry = match entry {
                    Ok(entry) => entry,
                    Err(_) => return None,
                };

                let file_name = entry.file_name();
                let file_name = match file_name.to_str() {
                    Some(file_name) => file_name,
                    None => return None,
                };

                if file_name.ends_with(".gitignore") {
                    Some(Template::new(file_name))
                } else {
                    None
                }
            })
            .collect();

        // Clippy throws a false negative here, probably caused by either of the two issues:
        // - https://github.com/rust-lang/rust-clippy/issues/5754
        // - https://github.com/rust-lang/rust-clippy/issues/6001
        #[allow(clippy::unnecessary_sort_by)]
        templates.sort_by(|a, b| a.name().cmp(b.name()));

        Ok(templates)
    }

    /// Update a repository with the latest templates from GitHub.
    ///
    /// When updating a repository, the latest templates are downloaded from GitHub and copied into
    /// the repository. This operation thus requires an internet connection.
    pub fn update(&self) -> Result<(), Error> {
        let archive = self.download_archive(None)?;
        self.extract_archive(&archive)?;

        Ok(())
    }

    /// Download an archive with all .gitignore templates.
    ///
    /// `alfred-gitignore` uses the `.gitignore` templates that GitHub provides in the
    /// [github/gitignore](https://github.com/github/gitignore) repository. An archive with with
    /// latest version of the `master` branch can be downloaded and stored at a temporary location
    /// using this function. It either returns the path to the file, or the error that occurred
    /// while attempting the download.
    fn download_archive(&self, url: Option<String>) -> Result<PathBuf, Error> {
        let url = url.unwrap_or_else(|| String::from(ARCHIVE));
        let mut response = reqwest::blocking::get(&url).unwrap();

        let file_path = temp_dir().join(
            PathBuf::from(ARCHIVE)
                .file_name()
                .unwrap_or_else(|| "archive.zip".as_ref()),
        );
        let mut destination = File::create(&file_path)?;

        copy(&mut response, &mut destination)?;

        Ok(file_path)
    }

    /// Extract an archive and write its files into a repository.
    ///
    /// An archive is a `.zip` file that contains many `.gitignore` files. When extracting it, these
    /// files are written to the given repository. While doing so, the directory structure is
    /// flattened. For example, `github/gitignore.gitignore` is written to `gitignore.gitignore` and
    /// the `github` directory is ignored.
    fn extract_archive(&self, archive: &PathBuf) -> Result<(), Error> {
        let file = File::open(archive)?;

        let mut archive = match ZipArchive::new(file) {
            Ok(archive) => archive,
            Err(error) => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    format!("Open ZIP archive failed with error '{}'", error),
                ))
            }
        };

        for i in 0..archive.len() {
            let mut file = match archive.by_index(i) {
                Ok(file) => file,
                Err(_) => continue, // TODO Log an error
            };

            let sanitized_name = PathBuf::from(file.name());

            let file_name = match sanitized_name.file_name() {
                Some(base_name) => base_name,
                None => continue, // TODO Log an error
            };

            let file_name_as_str = match file_name.to_str() {
                Some(name) => name,
                None => continue, // TODO Log an error
            };

            if file_name_as_str.ends_with("gitignore") {
                let file_path = self.path().join(file_name_as_str);
                let mut destination = File::create(file_path)?;

                copy(&mut file, &mut destination)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::repository::Repository;
    use crate::testing::initialize_repository;
    use mockito::{mock, Mock};
    use std::fs::{remove_file, File};
    use std::io::Write;
    use tempfile::TempDir;

    const ARCHIVE: &[u8] = include_bytes!("../tests/files/gitignore-master.zip");

    fn mock_get_archive() -> Mock {
        mock("GET", "/")
            .with_status(200)
            .with_header("Content-Type", "application/zip")
            .with_body(ARCHIVE.as_ref())
            .create()
    }

    #[test]
    fn new_with_existing_path() {
        let directory = TempDir::new().unwrap();
        let repository = Repository::new(directory.into_path());

        assert!(repository.is_ok())
    }

    #[test]
    fn new_with_empty_path() {
        let directory = TempDir::new().unwrap();
        let path = directory.path().join("does-not-exist");

        let repository = Repository::new(path.clone());

        assert!(repository.is_ok());
        assert!(path.exists());
    }

    #[test]
    fn download_archive() {
        let directory = TempDir::new().unwrap();
        let repository = Repository::new(directory.into_path()).unwrap();

        let _mock = mock_get_archive();

        let file = repository
            .download_archive(Some(mockito::server_url()))
            .unwrap();

        assert!(file.exists());
        assert_eq!("zip", file.extension().unwrap());
        assert_eq!(99108, file.metadata().unwrap().len());

        remove_file(file).unwrap();
    }

    #[test]
    fn extract_archive() {
        let repository_path = TempDir::new().unwrap();
        let repository = Repository::new(repository_path.into_path()).unwrap();

        let tempdir = TempDir::new().unwrap();
        let archive_path = tempdir.path().join("archive.zip");
        let mut archive = File::create(&archive_path).unwrap();
        archive.write_all(ARCHIVE).unwrap();
        archive.sync_all().unwrap();

        repository.extract_archive(&archive_path).unwrap();

        let gitignore_count = repository.templates().unwrap().len();
        assert_eq!(229, gitignore_count);
    }

    #[test]
    fn templates_returns_names() {
        let repository_path = TempDir::new().unwrap();
        let repository = initialize_repository(repository_path.path()).unwrap();

        let templates = repository.templates().unwrap();
        let template_names: Vec<&String> = templates.iter().map(|t| t.name()).collect();

        assert_eq!(vec!["apples", "oranges"], template_names);
    }
}
