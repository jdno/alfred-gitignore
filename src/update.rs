use crate::repository::Repository;
use std::env::temp_dir;
use std::fs::File;
use std::io::{copy, Error, ErrorKind};
use std::path::PathBuf;
use zip::ZipArchive;

const ARCHIVE: &str = "https://github.com/github/gitignore/archive/master.zip";

/// Update a repository with the latest templates from GitHub.
///
/// When updating a repository, the latest templates are downloaded from GitHub and copied into the
/// repository. This operation thus requires an internet connection.
pub fn update_repository(repository: &Repository) -> Result<(), Error> {
    let archive = download_archive(None)?;
    extract_archive(&archive, repository)?;

    Ok(())
}

/// Download an archive with all .gitignore templates.
///
/// `alfred-gitignore` uses the `.gitignore` templates that GitHub provides in the
/// [github/gitignore](https://github.com/github/gitignore) repository. An archive with with latest
/// version of the `master` branch can be downloaded and stored at a temporary location using this
/// function. It either returns the path to the file, or the error that occurred while attempting
/// the download.
fn download_archive(url: Option<String>) -> Result<PathBuf, Error> {
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
/// files are written to the given repository. While doing so, the directory structure is flattened.
/// For example, `github/gitignore.gitignore` is written to `gitignore.gitignore` and the `github`
/// directory is ignored.
fn extract_archive(archive: &PathBuf, repository: &Repository) -> Result<(), Error> {
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

        let sanitized_name = file.sanitized_name();

        let file_name = match sanitized_name.file_name() {
            Some(base_name) => base_name,
            None => continue, // TODO Log an error
        };

        let file_name_as_str = match file_name.to_str() {
            Some(name) => name,
            None => continue, // TODO Log an error
        };

        if file_name_as_str.ends_with("gitignore") {
            let file_path = repository.path().join(file_name_as_str);
            let mut destination = File::create(file_path)?;

            copy(&mut file, &mut destination)?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::repository::Repository;
    use crate::update::{download_archive, extract_archive};
    use mockito::{mock, Mock};
    use std::fs::{read_dir, remove_file, File};
    use std::io::Write;
    use tempdir::TempDir;

    const ARCHIVE: &[u8] = include_bytes!("../tests/files/gitignore-master.zip");

    fn mock_get_archive() -> Mock {
        mock("GET", "/")
            .with_status(200)
            .with_header("Content-Type", "application/zip")
            .with_body(ARCHIVE.as_ref())
            .create()
    }

    #[test]
    fn download_file_returns_zip() {
        let _mock = mock_get_archive();

        let file = download_archive(Some(mockito::server_url())).unwrap();

        assert!(file.exists());
        assert_eq!("zip", file.extension().unwrap());
        assert_eq!(99108, file.metadata().unwrap().len());

        remove_file(file).unwrap();
    }

    #[test]
    fn extract_archive_populates_repository() {
        let repository_path = TempDir::new("alfred-gitignore").unwrap();
        let repository = Repository::new(repository_path.into_path()).unwrap();

        let tempdir = TempDir::new("alfred-gitignore").unwrap();
        let archive_path = tempdir.path().join("archive.zip");
        let mut archive = File::create(&archive_path).unwrap();
        archive.write_all(ARCHIVE).unwrap();
        archive.sync_all().unwrap();

        extract_archive(&archive_path, &repository).unwrap();

        let gitignore_count = read_dir(repository.path())
            .unwrap()
            .map(|entry| entry.unwrap())
            .filter(|entry| entry.file_name().to_str().unwrap().ends_with(".gitignore"))
            .count();

        assert_eq!(229, gitignore_count);
    }
}
