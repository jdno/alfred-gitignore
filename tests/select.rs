use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tempdir::TempDir;

fn write_templates(path: &Path) {
    let mut apples = File::create(path.join("Apples.gitignore")).unwrap();
    apples
        .write_all(include_bytes!("files/repository/apples.gitignore"))
        .unwrap();
    apples.sync_all().unwrap();

    let mut oranges = File::create(path.join("Oranges.gitignore")).unwrap();
    oranges
        .write_all(include_bytes!("files/repository/oranges.gitignore"))
        .unwrap();
    oranges.sync_all().unwrap();
}

#[test]
fn select_with_suggestions() {
    let mut command = Command::cargo_bin("alfred-gitignore").unwrap();

    let repository = TempDir::new("alfred-gitignore").unwrap();
    write_templates(repository.path());

    command.arg("--repository").arg(repository.path());
    command.arg("--select");
    command.arg("apples").arg("o");

    command
        .assert()
        .success()
        .stdout(predicate::str::contains("--select Apples Oranges"));
}

#[test]
fn select_without_input() {
    let mut command = Command::cargo_bin("alfred-gitignore").unwrap();

    let repository = TempDir::new("alfred-gitignore").unwrap();
    write_templates(repository.path());

    command.arg("--repository").arg(repository.path());
    command.arg("--select");

    command.assert().success().stdout(
        predicate::str::contains("--select Apples")
            .and(predicate::str::contains("--select Oranges")),
    );
}
