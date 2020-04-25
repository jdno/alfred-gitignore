use assert_cmd::Command;
use predicates::prelude::*;
use tempdir::TempDir;

#[test]
fn default_without_args() {
    let mut command = Command::cargo_bin("alfred-gitignore").unwrap();

    let repository = TempDir::new("alfred-gitignore").unwrap();
    command.arg("--repository").arg(repository.path());

    command
        .assert()
        .success()
        .stdout(predicate::str::contains("--select").and(predicate::str::contains("--update")));
}
