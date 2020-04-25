use crate::repository::Repository;
use std::fs::File;
use std::io::{Error, Write};
use std::path::{Path, PathBuf};

/// Initializes a repository for testing.
///
/// This function initializes a repository for testing at the given location. The repository is
/// populated with the two `.gitignore` templates in `tests/files/repository`.
pub fn initialize_repository(path: &Path) -> Result<Repository, Error> {
    let repository = Repository::new(PathBuf::from(path))?;

    let mut apples = File::create(&repository.path().join("apples.gitignore"))?;
    apples.write_all(include_bytes!("../tests/files/repository/apples.gitignore"))?;
    apples.sync_all()?;

    let mut oranges = File::create(&repository.path().join("oranges.gitignore"))?;
    oranges.write_all(include_bytes!(
        "../tests/files/repository/oranges.gitignore"
    ))?;
    oranges.sync_all()?;

    Ok(repository)
}
